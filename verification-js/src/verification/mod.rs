use csaf::Csaf;
use csaf_walker::verification::check::vex::check_all_products_v11ies_exits_in_product_tree;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn verifiy_csaf() {
    let csaf: Csaf =
        serde_json::from_str(include_str!("../../../csaf/test-data/rhsa-2021_3029.json"))
            .expect("example data must parse");
    let es = check_all_products_v11ies_exits_in_product_tree(&csaf);
    for e in es {
        println!("{:?}", e);
    }
}
