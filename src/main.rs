use Rust_Trees::rbtree::RBTree;
use Rust_Trees::avltree::AvlTree;
use Rust_Trees::bst::BST;
use std::env;
use std::io;

#[allow(non_snake_case)]

fn main() {
    let args: Vec<String> = env::args().collect();
    let choice = args.get(1);
    match choice {
        Some(n) => {
            if n == "rb" {
                println!("You selected Red Black Tree!");
                run_rbtree();
            } else if n == "avl" {
                println!("You selected AVL Tree!");
                run_avltree();
            } else {
                println!("Invalid input. Please select either rb or avl (Ex. cargo run rb or cargon run avl).");
            }
        },
        None => {println!("Invalid input. Please select either rb or avl (Ex. cargo run rb or cargon run avl).");}
    }
}

fn run_rbtree() {
    let mut tree: RBTree<u32> = RBTree::<u32>::new();
    let mut option: u32;
    loop {
        print_options();
        option = handle_user_input();
        let num: u32;
        match option {
            0 => {
                println!("Exiting Application......");
                break;
            }
            1 => {
                // insert node
                println!("Enter a number you would like to insert: ");
                num = handle_user_input();
                tree.insert(num);
            },
            2 => {
                // delete node
                println!("Enter key of node you would like to delete: ");
                num = handle_user_input();
                tree.delete(num);
            },
            3 => {
                // count leaves
                println!(" ");
                println!("PRINT OUTPUT ----------------------------------------------- ");
                println!("There are {} leaf node(s) in the tree.", tree.leaves());
                println!("------------------------------------------------------ ");
                println!(" ");
            },
            4 => {
                // count leaves
                println!(" ");
                println!("PRINT OUTPUT ----------------------------------------------- ");
                println!("The height of the tree is {}", tree.height());
                println!("------------------------------------------------------ ");
                println!(" ");
            },
            5 => {
                // in-order traversal
                println!(" ");
                println!("PRINT OUTPUT ----------------------------------------------- ");
                tree.print_inorder();
                println!("------------------------------------------------------ ");
                println!(" ");
            },
            6 => {
                // is tree empty
                println!(" ");
                println!("PRINT OUTPUT ----------------------------------------------- ");
                println!("Is tree empty? {}", tree.is_empty());
                println!("------------------------------------------------------ ");
                println!(" ");
            },
            7 => {
                // print the tree
                println!(" ");
                println!("PRINT OUTPUT ----------------------------------------------- ");
                //println!("{:#?}",tree);
                tree.print_tree();
                println!("------------------------------------------------------ ");
                println!(" ");
            },
            8 => {
                // number of nodes
                println!(" ");
                println!("PRINT OUTPUT ----------------------------------------------- ");
                println!("There are {} node(s) in the tree", tree.count());
                println!("------------------------------------------------------ ");
                println!(" ");
            },
            9 => {
                // search for a node
                println!("Enter key of node you would like to search for: ");
                num = handle_user_input();
                println!(" ");
                println!("PRINT OUTPUT ----------------------------------------------- ");
                println!("{:#?}", tree.search(num));
                println!("------------------------------------------------------ ");
                println!(" ");
            },
            10 => {
                // find minimum
                println!(" ");
                println!("PRINT OUTPUT ----------------------------------------------- ");
                println!("{:#?}", tree.min());
                println!("------------------------------------------------------ ");
                println!(" ");
            },
            11 => {
                // find maximum
                println!(" ");
                println!("PRINT OUTPUT ----------------------------------------------- ");
                println!("{:#?}", tree.max());
                println!("------------------------------------------------------ ");
                println!(" ");
            },
            12 => {
                // pre-order traversal
                println!(" ");
                println!("PRINT OUTPUT ----------------------------------------------- ");
                tree.print_preorder();
                println!("------------------------------------------------------ ");
                println!(" ");
            },
            13 => {
                // in-order traversal
                println!(" ");
                println!("PRINT OUTPUT ----------------------------------------------- ");
                tree.print_inorder();
                println!("------------------------------------------------------ ");
                println!(" ");
            },
            _ => {break;}
        }
    };
}

fn run_avltree() {
    let mut tree: AvlTree<u32> = AvlTree::<u32>::new();
    let mut option: u32;
    loop {
        print_options();
        option = handle_user_input();
        let num: u32;
        match option {
            0 => {
                println!("Exiting Application......");
                break;
            }
            1 => {
                // insert node
                println!("Enter a number you would like to insert: ");
                num = handle_user_input();
                tree.insert(num);
            },
            2 => {
                // delete node
                println!("Enter key of node you would like to delete: ");
                num = handle_user_input();
                tree.delete(num);
            },
            3 => {
                // count leaves
                println!(" ");
                println!("PRINT OUTPUT ----------------------------------------------- ");
                println!("There are {} leaf node(s) in the tree.", tree.leaves());
                println!("------------------------------------------------------ ");
                println!(" ");
            },
            4 => {
                // return height
                // height function needs to be public
                println!(" ");
                println!("PRINT OUTPUT ----------------------------------------------- ");
                println!("The height of the tree is {}", tree.height());
                println!("------------------------------------------------------ ");
                println!(" ");
            },
            5 => {
                // in-order traversal
                println!(" ");
                println!("PRINT OUTPUT ----------------------------------------------- ");
                tree.print_inorder();
                println!("------------------------------------------------------ ");
                println!(" ");
            },
            6 => {
                // is tree empty
                println!(" ");
                println!("PRINT OUTPUT ----------------------------------------------- ");
                println!("Is tree empty? {}", tree.is_empty());
                println!("------------------------------------------------------ ");
                println!(" ");
            },
            7 => {
                // print the tree
                println!(" ");
                println!("PRINT OUTPUT ----------------------------------------------- ");
                tree.print_tree();
                println!("------------------------------------------------------ ");
                println!(" ");
            },
            8 => {
                // number of nodes
                println!(" ");
                println!("PRINT OUTPUT ----------------------------------------------- ");
                println!("There are {} node(s) in the tree", tree.count());
                println!("------------------------------------------------------ ");
                println!(" ");
            },
            9 => {
                // search for a node
                println!("Enter key of node you would like to search for: ");
                num = handle_user_input();
                println!(" ");
                println!("PRINT OUTPUT ----------------------------------------------- ");
                println!("{:#?}", tree.search(num));
                println!("------------------------------------------------------ ");
                println!(" ");
            },
            10 => {
                // find minimum
                println!(" ");
                println!("PRINT OUTPUT ----------------------------------------------- ");
                println!("{:#?}", tree.min());
                println!("------------------------------------------------------ ");
                println!(" ");
            },
            11 => {
                // find maximum
                println!(" ");
                println!("PRINT OUTPUT ----------------------------------------------- ");
                println!("{:#?}", tree.max());
                println!("------------------------------------------------------ ");
                println!(" ");
            },
            12 => {
                // pre-order traversal
                println!(" ");
                println!("PRINT OUTPUT ----------------------------------------------- ");
                tree.print_preorder();
                println!("------------------------------------------------------ ");
                println!(" ");
            },
            13 => {
                // in-order traversal
                println!(" ");
                println!("PRINT OUTPUT ----------------------------------------------- ");
                tree.print_inorder();
                println!("------------------------------------------------------ ");
                println!(" ");
            },
            _ => {break;}
        }
    };
}

fn print_options() {
    println!("Select one of the following options: ");
    println!("0 to exit application");
    println!("1 to insert node");
    println!("2 to delete node");
    println!("3 to count number of leaves");
    println!("4 to return the height");
    println!("5 to print in-order traversal");
    println!("6 to check if the tree is empty");
    println!("7 to print the tree");
    println!("8 to return number of nodes");
    println!("9 to search for a node");
    println!("10 to find minimum of tree");
    println!("11 to find maximum of tree");
    println!("12 to print pre-order traversal");
    println!("13 to print level-order traversal");
    println!(" ");
}

fn handle_user_input() -> u32 {
    let mut selection = String::new();
    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read line");
    
    let num;
    match selection.trim().parse() {
        Ok(x) => {num = x;}
        Err(_) => {
            panic!("Invalid input. Please try again.")
        }
    }
    num
}
