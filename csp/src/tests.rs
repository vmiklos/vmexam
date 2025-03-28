/*
 * Copyright 2025 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
*/

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Tests the csp library crate.

use super::*;

#[test]
fn test_cube() {
    let problem = r#"red,purple,green,blue,brown,yellow
brown,blue,green,red,yellow,purple
brown,purple,yellow,green,red,blue
brown,yellow,red,green,blue,purple
yellow,blue,green,purple,red,brown
yellow,green,brown,blue,red,purple
blue,brown,purple,green,red,yellow
blue,yellow,brown,green,red,purple"#;
    let mut model = cube::Model::new(&problem);

    let ret = model.solve();

    assert!(ret);
    assert_eq!(model.get_cube_index(cube::SLOT_DFL), 7);
    assert_eq!(
        model.get_color_string(cube::SLOT_DFL, cube::SIDE_U),
        "green"
    );
    assert_eq!(
        model.get_color_string(cube::SLOT_DFL, cube::SIDE_F),
        "yellow"
    );
    assert_eq!(model.get_cube_index(cube::SLOT_DFR), 5);
    assert_eq!(
        model.get_color_string(cube::SLOT_DFL, cube::SIDE_U),
        "green"
    );
    assert_eq!(
        model.get_color_string(cube::SLOT_DFL, cube::SIDE_F),
        "yellow"
    );
    assert_eq!(model.get_cube_index(cube::SLOT_DBR), 4);
    assert_eq!(model.get_color_string(cube::SLOT_DBR, cube::SIDE_U), "blue");
    assert_eq!(
        model.get_color_string(cube::SLOT_DBR, cube::SIDE_F),
        "yellow"
    );
    assert_eq!(model.get_cube_index(cube::SLOT_DBL), 8);
    assert_eq!(model.get_color_string(cube::SLOT_DBL, cube::SIDE_U), "red");
    assert_eq!(
        model.get_color_string(cube::SLOT_DBL, cube::SIDE_F),
        "green"
    );
    assert_eq!(model.get_cube_index(cube::SLOT_UBL), 1);
    assert_eq!(
        model.get_color_string(cube::SLOT_UBL, cube::SIDE_U),
        "purple"
    );
    assert_eq!(
        model.get_color_string(cube::SLOT_UBL, cube::SIDE_F),
        "yellow"
    );
    assert_eq!(model.get_cube_index(cube::SLOT_UBR), 2);
    assert_eq!(
        model.get_color_string(cube::SLOT_UBR, cube::SIDE_U),
        "purple"
    );
    assert_eq!(model.get_color_string(cube::SLOT_UBR, cube::SIDE_F), "blue");
    assert_eq!(model.get_cube_index(cube::SLOT_UFR), 3);
    assert_eq!(
        model.get_color_string(cube::SLOT_UFR, cube::SIDE_U),
        "purple"
    );
    assert_eq!(
        model.get_color_string(cube::SLOT_UFR, cube::SIDE_F),
        "yellow"
    );
    assert_eq!(model.get_cube_index(cube::SLOT_UFL), 6);
    assert_eq!(
        model.get_color_string(cube::SLOT_UFL, cube::SIDE_U),
        "purple"
    );
    assert_eq!(
        model.get_color_string(cube::SLOT_UFL, cube::SIDE_F),
        "yellow"
    );
}
