use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProductRequest {
    #[validate(length(min = 1, max = 255, message = "Name must be between 1 and 255 characters"))]
    pub name: String,
    pub description: Option<String>,
    pub price: Decimal,
    #[validate(range(min = 0, message = "Quantity must be non-negative"))]
    pub quantity: i32,
    pub category: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<Decimal>,
    pub quantity: Option<i32>,
    pub category: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProductResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub quantity: i32,
    pub category: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct ProductListResponse {
    pub products: Vec<ProductResponse>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
}


#[cfg(test)]
mod tests {
    use super::*;
    use mockall::{automock, predicate::*};
    use rust_decimal_macros::dec;
    use validator::Validate;

    // Step 1: Mockable interface for validation (so services could mock validation behavior)
    #[automock]
    trait ProductValidator {
        fn validate_create(&self, req: &CreateProductRequest) -> Result<(), validator::ValidationErrors>;
    }

    struct RealProductValidator;

    impl ProductValidator for RealProductValidator {
        fn validate_create(&self, req: &CreateProductRequest) -> Result<(), validator::ValidationErrors> {
            req.validate()
        }
    }

    #[test]
    fn test_create_product_request_valid() {
        let request = CreateProductRequest {
            name: "Laptop".to_string(),
            description: Some("A gaming laptop".to_string()),
            price: dec!(1299.99),
            quantity: 10,
            category: Some("Electronics".to_string()),
        };

        let result = request.validate();
        assert!(result.is_ok(), "Expected valid request, got {:?}", result.err());
    }

    #[test]
    fn test_create_product_request_invalid_name() {
        let request = CreateProductRequest {
            name: "".to_string(), // too short
            description: None,
            price: dec!(10.0),
            quantity: 5,
            category: None,
        };

        let result = request.validate();
        assert!(result.is_err(), "Expected validation error for empty name");
        let errors = result.unwrap_err();
        assert!(errors.to_string().contains("Name must be between 1 and 255 characters"));
    }

    #[test]
    fn test_create_product_request_invalid_quantity() {
        let request = CreateProductRequest {
            name: "Mouse".to_string(),
            description: None,
            price: dec!(25.00),
            quantity: -5, // negative not allowed
            category: None,
        };

        let result = request.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.to_string().contains("Quantity must be non-negative"));
    }

    #[test]
    fn test_mock_product_validator_success() {
        let mut mock = MockProductValidator::new();
        let req = CreateProductRequest {
            name: "Keyboard".to_string(),
            description: None,
            price: dec!(45.00),
            quantity: 2,
            category: None,
        };

        mock.expect_validate_create()
            .with(always())
            .returning(|_| Ok(()));

        let result = mock.validate_create(&req);
        assert!(result.is_ok(), "Expected mock to return Ok");
    }

    #[test]
    fn test_mock_product_validator_failure() {
        let mut mock = MockProductValidator::new();
        let req = CreateProductRequest {
            name: "".to_string(),
            description: None,
            price: dec!(10.0),
            quantity: 1,
            category: None,
        };

        let mut errors = validator::ValidationErrors::new();
        errors.add("name", validator::ValidationError::new("empty").into());

        mock.expect_validate_create()
            .with(always())
            .returning(move |_| Err(errors.clone()));

        let result = mock.validate_create(&req);
        assert!(result.is_err(), "Expected mock to return Err");
        assert!(result.unwrap_err().to_string().contains("name"));
    }
}
