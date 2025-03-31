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

    let ret = model.solve().unwrap();

    assert!(ret);
    assert_eq!(model.get_cube_index(cube::Slot::DFL), 7);
    assert_eq!(
        model
            .get_color_string(cube::Slot::DFL, cube::SIDE_U)
            .unwrap(),
        "green"
    );
    assert_eq!(
        model
            .get_color_string(cube::Slot::DFL, cube::SIDE_F)
            .unwrap(),
        "yellow"
    );
    assert_eq!(model.get_cube_index(cube::Slot::DFR), 5);
    assert_eq!(
        model
            .get_color_string(cube::Slot::DFL, cube::SIDE_U)
            .unwrap(),
        "green"
    );
    assert_eq!(
        model
            .get_color_string(cube::Slot::DFL, cube::SIDE_F)
            .unwrap(),
        "yellow"
    );
    assert_eq!(model.get_cube_index(cube::Slot::DBR), 4);
    assert_eq!(
        model
            .get_color_string(cube::Slot::DBR, cube::SIDE_U)
            .unwrap(),
        "blue"
    );
    assert_eq!(
        model
            .get_color_string(cube::Slot::DBR, cube::SIDE_F)
            .unwrap(),
        "yellow"
    );
    assert_eq!(model.get_cube_index(cube::Slot::DBL), 8);
    assert_eq!(
        model
            .get_color_string(cube::Slot::DBL, cube::SIDE_U)
            .unwrap(),
        "red"
    );
    assert_eq!(
        model
            .get_color_string(cube::Slot::DBL, cube::SIDE_F)
            .unwrap(),
        "green"
    );
    assert_eq!(model.get_cube_index(cube::Slot::UBL), 1);
    assert_eq!(
        model
            .get_color_string(cube::Slot::UBL, cube::SIDE_U)
            .unwrap(),
        "purple"
    );
    assert_eq!(
        model
            .get_color_string(cube::Slot::UBL, cube::SIDE_F)
            .unwrap(),
        "yellow"
    );
    assert_eq!(model.get_cube_index(cube::Slot::UBR), 2);
    assert_eq!(
        model
            .get_color_string(cube::Slot::UBR, cube::SIDE_U)
            .unwrap(),
        "purple"
    );
    assert_eq!(
        model
            .get_color_string(cube::Slot::UBR, cube::SIDE_F)
            .unwrap(),
        "blue"
    );
    assert_eq!(model.get_cube_index(cube::Slot::UFR), 3);
    assert_eq!(
        model
            .get_color_string(cube::Slot::UFR, cube::SIDE_U)
            .unwrap(),
        "purple"
    );
    assert_eq!(
        model
            .get_color_string(cube::Slot::UFR, cube::SIDE_F)
            .unwrap(),
        "yellow"
    );
    assert_eq!(model.get_cube_index(cube::Slot::UFL), 6);
    assert_eq!(
        model
            .get_color_string(cube::Slot::UFL, cube::SIDE_U)
            .unwrap(),
        "purple"
    );
    assert_eq!(
        model
            .get_color_string(cube::Slot::UFL, cube::SIDE_F)
            .unwrap(),
        "yellow"
    );
}
