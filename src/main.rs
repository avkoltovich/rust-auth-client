use rust_auth_client::{auth::auth_module::{get_access_token}, requests::requests::UserData, tree::build_tree, requests::requests::{RawHierarchy, get_hierarchy, get_user_info}};

fn main() {
    let access_token = get_access_token();

    // let _user_data: UserData = get_user_info(&access_token, 1388).unwrap();

    let hierarchy_view: RawHierarchy = get_hierarchy(&access_token, 4).unwrap();

    let tree = build_tree(Box::new(&hierarchy_view.hierarchy));

    println!("\n{:#?}", tree);
}
