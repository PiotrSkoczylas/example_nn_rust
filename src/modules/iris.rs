use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize, Clone)]
pub struct Iris {
    sepal_length: f64,
    sepal_width: f64,
    petal_length: f64,
    petal_width: f64,
    variety: String,
}

impl fmt::Display for Iris {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}, {}, {}, {})",
            self.sepal_length, self.sepal_width, self.petal_length, self.petal_width, self.variety
        )
    }
}

impl Iris {
    pub fn get_vec(&self) -> Vec<f64> {
        let mut result: Vec<f64> = Vec::new();
        result.push(self.sepal_length);
        result.push(self.sepal_width);
        result.push(self.petal_length);
        result.push(self.petal_width);
        result
    }

    pub fn class_num(&self) -> Vec<f64> {
        let mut vari: Vec<f64> = vec![0.0, 0.0, 0.0];
        if self.variety.eq("Setosa") {
            vari[0] = 1.0;
        } else if self.variety.eq("Virginica") {
            vari[1] = 1.0;
        } else if self.variety.eq("Versicolor") {
            vari[2] = 1.0;
        }
        vari
    }
}
