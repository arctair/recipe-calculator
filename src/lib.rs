/// Calculate raw cost of a recipe
///
/// ```
/// let recipe = recipe_calculator::Recipe {
///     produces: recipe_calculator::NamedQuantity {
///         name: String::from("wood planks"),
///         quantity: 2,
///     },
///     ingredients: recipe_calculator::NamedQuantity {
///         name: String::from("wood"),
///         quantity: 1,
///     },
/// };
/// let actual_raw_cost = recipe_calculator::raw_cost(
///     recipe,
///     recipe_calculator::NamedQuantity {
///         name: String::from("wood planks"),
///         quantity: 64,
///     },
/// );
/// let expected_raw_cost = recipe_calculator::NamedQuantity {
///     name: String::from("wood"),
///     quantity: 32,
/// };
/// assert_eq!(actual_raw_cost, expected_raw_cost);
/// ```
pub fn raw_cost(recipe: Recipe, named_quantity: NamedQuantity) -> NamedQuantity {
    NamedQuantity {
        name: recipe.ingredients.name,
        quantity: named_quantity.quantity / recipe.produces.quantity
    }
}

#[derive(PartialEq, Debug)]
pub struct Recipe{
    pub produces: NamedQuantity,
    pub ingredients: NamedQuantity,
}

#[derive(PartialEq, Debug)]
pub struct NamedQuantity {
    pub name: String,
    pub quantity: i32,
}
