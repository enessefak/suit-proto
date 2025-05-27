pub mod projectsuit {
    // 1. Common Modülü
    pub mod common {
        pub mod v1 {
            include!("gen/projectsuit.common.v1.rs");
        }
    }

    // 2. Unit Modülü
    pub mod unit {
        pub mod v1 {
            include!("gen/projectsuit.unit.v1.rs");
        }
    }

    // 3. Category Modülü
    pub mod category {
        pub mod v1 {
            include!("gen/projectsuit.category.v1.rs");
        }
    }

    // 4. Product Modülü
    pub mod product {
        pub mod v1 {
            include!("gen/projectsuit.product.v1.rs");
        }
    }

    // 5. Material Modülü
    pub mod material {
        pub mod v1 {
            include!("gen/projectsuit.material.v1.rs");
        }
    }

    // 6. Tax Modülü
    pub mod tax {
        pub mod v1 {
            include!("gen/projectsuit.tax.v1.rs");
        }
    }

    // 7. Menu Modülü
    pub mod menu {
        pub mod v1 {
            include!("gen/projectsuit.menu.v1.rs");
        }
    }

    // 8. Pricing Modülü
    pub mod pricing {
        pub mod v1 {
            include!("gen/projectsuit.pricing.v1.rs");
        }
    }

    // 9. Customer Modülü (Yeni Eklendi)
    pub mod customer {
        pub mod v1 {
            include!("gen/projectsuit.customer.v1.rs");
        }
    }

    // 10. Inventory Modülü (Yeni Eklendi)
    pub mod inventory {
        pub mod v1 {
            include!("gen/projectsuit.inventory.v1.rs");
        }
    }

    // 11. Device Modülü (Yeni Eklendi)
    pub mod device {
        pub mod v1 {
            include!("gen/projectsuit.device.v1.rs");
        }
    }

    // 12. Restaurant Modülü (Yeni Eklendi)
    pub mod restaurant {
        pub mod v1 {
            include!("gen/projectsuit.restaurant.v1.rs");
        }
    }

    // 13. Hotel Modülü (Yeni Eklendi)
    pub mod hotel {
        pub mod v1 {
            include!("gen/projectsuit.hotel.v1.rs");
        }
    }

    // 14. Accounting Modülü (Yeni Eklendi)
    pub mod accounting {
        pub mod v1 {
            include!("gen/projectsuit.accounting.v1.rs");
        }
    }

    // 15. Payment Modülü (Yeni Eklendi)
    pub mod payment {
        pub mod v1 {
            include!("gen/projectsuit.payment.v1.rs");
        }
    }

    // 16. SalesChannel Modülü (Yeni Eklendi)
    pub mod sales_channel {
        pub mod v1 {
            include!("gen/projectsuit.sales_channel.v1.rs");
        }
    }

    // 17. User Modülü (Yeni Eklendi)
    pub mod user {
        pub mod v1 {
            include!("gen/projectsuit.user.v1.rs");
        }
    }
    
    // 18. Order Modülü (Mevcuttu, yeri korunuyor)
    pub mod order {
        pub mod v1 {
            include!("gen/projectsuit.order.v1.rs");
        }
    }

    // 19. Sync Modülü (Mevcuttu, yeri korunuyor)
    pub mod sync {
        pub mod v1 {
            include!("gen/projectsuit.sync.v1.rs");
        }
    }
}