use serde::Deserialize;

struct Calculator {
    recipe: Recipe,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Recipe {
    pub consumes: NamedQuantity,
    pub produces: NamedQuantity,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct NamedQuantity {
    pub name: String,
    pub quantity: i32,
}

impl Calculator {
    pub fn new(toml_string: String) -> Self {
        Self {
            recipe: toml::from_str(&toml_string).unwrap(),
        }
    }

    pub fn raw_cost(&self, query: String) -> String {
        format!("{} {}", self.recipe.consumes.quantity, self.recipe.consumes.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiply_quantity() {
        let recipe_toml = String::from(
            r#"
                consumes = { name = "wood", quantity = 1 }
                produces = { name = "wood planks", quantity = 2 }
            "#,
        );

        let calculator = Calculator::new(recipe_toml);

        let expected_raw_cost = "1 wood";
        let actual_raw_cost = calculator.raw_cost(String::from("2 wood planks"));
        assert_eq!(expected_raw_cost, actual_raw_cost)
    }
}
