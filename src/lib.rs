//! Rust client for [FishFYI](https://fishfyi.com) REST API.
//!
//! ```rust
//! let client = fishfyi::Client::new();
//! let result = client.search("query").unwrap();
//! ```

use serde_json::Value;

pub struct Client {
    base_url: String,
    http: reqwest::blocking::Client,
}

impl Client {
    pub fn new() -> Self {
        Self {
            base_url: "https://fishfyi.com".to_string(),
            http: reqwest::blocking::Client::new(),
        }
    }

    fn get(&self, path: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self.http.get(&url).send()?.json()?;
        Ok(resp)
    }

    pub fn search(&self, query: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/api/v1/search/?q={}", self.base_url, query);
        let resp = self.http.get(&url).send()?.json()?;
        Ok(resp)
    }

    /// List all compatibility.
    pub fn list_compatibility(&self) -> Result<Value, Box<dyn std::error::Error>> {
        self.get("/api/v1/compatibility/")
    }

    /// Get compatibility by slug.
    pub fn get_compatibility(&self, slug: &str) -> Result<Value, Box<dyn std::error::Error>> {
        self.get(&format!("/api/v1/compatibility/{}/", slug))
    }
    /// List all countries.
    pub fn list_countries(&self) -> Result<Value, Box<dyn std::error::Error>> {
        self.get("/api/v1/countries/")
    }

    /// Get country by slug.
    pub fn get_country(&self, slug: &str) -> Result<Value, Box<dyn std::error::Error>> {
        self.get(&format!("/api/v1/countries/{}/", slug))
    }
    /// List all families.
    pub fn list_families(&self) -> Result<Value, Box<dyn std::error::Error>> {
        self.get("/api/v1/families/")
    }

    /// Get family by slug.
    pub fn get_family(&self, slug: &str) -> Result<Value, Box<dyn std::error::Error>> {
        self.get(&format!("/api/v1/families/{}/", slug))
    }
    /// List all faqs.
    pub fn list_faqs(&self) -> Result<Value, Box<dyn std::error::Error>> {
        self.get("/api/v1/faqs/")
    }

    /// Get faq by slug.
    pub fn get_faq(&self, slug: &str) -> Result<Value, Box<dyn std::error::Error>> {
        self.get(&format!("/api/v1/faqs/{}/", slug))
    }
    /// List all fish.
    pub fn list_fish(&self) -> Result<Value, Box<dyn std::error::Error>> {
        self.get("/api/v1/fish/")
    }

    /// Get fish by slug.
    pub fn get_fish(&self, slug: &str) -> Result<Value, Box<dyn std::error::Error>> {
        self.get(&format!("/api/v1/fish/{}/", slug))
    }
    /// List all glossary.
    pub fn list_glossary(&self) -> Result<Value, Box<dyn std::error::Error>> {
        self.get("/api/v1/glossary/")
    }

    /// Get term by slug.
    pub fn get_term(&self, slug: &str) -> Result<Value, Box<dyn std::error::Error>> {
        self.get(&format!("/api/v1/glossary/{}/", slug))
    }
    /// List all glossary categories.
    pub fn list_glossary_categories(&self) -> Result<Value, Box<dyn std::error::Error>> {
        self.get("/api/v1/glossary-categories/")
    }

    /// Get glossary category by slug.
    pub fn get_glossary_category(&self, slug: &str) -> Result<Value, Box<dyn std::error::Error>> {
        self.get(&format!("/api/v1/glossary-categories/{}/", slug))
    }
    /// List all guide series.
    pub fn list_guide_series(&self) -> Result<Value, Box<dyn std::error::Error>> {
        self.get("/api/v1/guide-series/")
    }

    /// Get guide sery by slug.
    pub fn get_guide_sery(&self, slug: &str) -> Result<Value, Box<dyn std::error::Error>> {
        self.get(&format!("/api/v1/guide-series/{}/", slug))
    }
    /// List all guides.
    pub fn list_guides(&self) -> Result<Value, Box<dyn std::error::Error>> {
        self.get("/api/v1/guides/")
    }

    /// Get guide by slug.
    pub fn get_guide(&self, slug: &str) -> Result<Value, Box<dyn std::error::Error>> {
        self.get(&format!("/api/v1/guides/{}/", slug))
    }
    /// List all methods.
    pub fn list_methods(&self) -> Result<Value, Box<dyn std::error::Error>> {
        self.get("/api/v1/methods/")
    }

    /// Get method by slug.
    pub fn get_method(&self, slug: &str) -> Result<Value, Box<dyn std::error::Error>> {
        self.get(&format!("/api/v1/methods/{}/", slug))
    }
    /// List all orders.
    pub fn list_orders(&self) -> Result<Value, Box<dyn std::error::Error>> {
        self.get("/api/v1/orders/")
    }

    /// Get order by slug.
    pub fn get_order(&self, slug: &str) -> Result<Value, Box<dyn std::error::Error>> {
        self.get(&format!("/api/v1/orders/{}/", slug))
    }
    /// List all regions.
    pub fn list_regions(&self) -> Result<Value, Box<dyn std::error::Error>> {
        self.get("/api/v1/regions/")
    }

    /// Get region by slug.
    pub fn get_region(&self, slug: &str) -> Result<Value, Box<dyn std::error::Error>> {
        self.get(&format!("/api/v1/regions/{}/", slug))
    }
    /// List all seasons.
    pub fn list_seasons(&self) -> Result<Value, Box<dyn std::error::Error>> {
        self.get("/api/v1/seasons/")
    }

    /// Get season by slug.
    pub fn get_season(&self, slug: &str) -> Result<Value, Box<dyn std::error::Error>> {
        self.get(&format!("/api/v1/seasons/{}/", slug))
    }
    /// List all water bodies.
    pub fn list_water_bodies(&self) -> Result<Value, Box<dyn std::error::Error>> {
        self.get("/api/v1/water-bodies/")
    }

    /// Get water body by slug.
    pub fn get_water_body(&self, slug: &str) -> Result<Value, Box<dyn std::error::Error>> {
        self.get(&format!("/api/v1/water-bodies/{}/", slug))
    }
}

impl Default for Client {
    fn default() -> Self { Self::new() }
}
