use rust_auth_client::{auth::{get_access_token}, models::{RawHierarchy, UserData}, requests::{get_hierarchy, get_user_info}, tree::build_tree};

fn main() {
    let access_token = get_access_token();

    let user_data: UserData = get_user_info(&access_token, 1388).unwrap();

    let _hierarchy_view: RawHierarchy = get_hierarchy(&access_token, 4).unwrap();

    // let tree = build_tree(Box::new(&hierarchy_view.hierarchy));

    println!("\n{:#?}", user_data);
}
