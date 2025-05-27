pub mod projectsuit {
    pub mod common {
        pub mod v1 {
            include!("gen/projectsuit.common.v1.rs");
        }
    }
    pub mod category {
        pub mod v1 {
            include!("gen/projectsuit.category.v1.rs");
        }
    }
    pub mod material {
        pub mod v1 {
            include!("gen/projectsuit.material.v1.rs");
        }
    }
    pub mod menu {
        pub mod v1 {
            include!("gen/projectsuit.menu.v1.rs");
        }
    }
    pub mod order {
        pub mod v1 {
            include!("gen/projectsuit.order.v1.rs");
        }
    }
    pub mod pricing {
        pub mod v1 {
            include!("gen/projectsuit.pricing.v1.rs");
        }
    }
    pub mod product {
        pub mod v1 {
            include!("gen/projectsuit.product.v1.rs");
        }
    }
    pub mod recipe {
        pub mod v1 {
            include!("gen/projectsuit.recipe.v1.rs");
        }
    }
    pub mod sync {
        pub mod v1 {
            include!("gen/projectsuit.sync.v1.rs");
        }
    }
    pub mod tax {
        pub mod v1 {
            include!("gen/projectsuit.tax.v1.rs");
        }
    }
    pub mod unit {
        pub mod v1 {
            include!("gen/projectsuit.unit.v1.rs");
        }
    }
}
