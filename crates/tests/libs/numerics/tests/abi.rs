use std::mem::{align_of, size_of, transmute};
use windows_numerics::{Matrix3x2, Matrix4x4, Vector2, Vector3, Vector4};
use windows_sys::Win32::{
    d3d9::{D3DMATRIX, D3DMATRIX_0},
    dcommon::{
        D2D_MATRIX_3X2_F, D2D_MATRIX_3X2_F_0, D2D_MATRIX_4X4_F, D2D_MATRIX_4X4_F_0, D2D_POINT_2F,
        D2D_VECTOR_2F, D2D_VECTOR_3F, D2D_VECTOR_4F,
    },
};

fn assert_abi<T, U>() {
    assert_eq!(size_of::<T>(), size_of::<U>());
    assert_eq!(align_of::<T>(), align_of::<U>());
}

#[test]
fn native_numeric_types_match_windows_numerics() {
    assert_abi::<D2D_MATRIX_3X2_F, Matrix3x2>();
    assert_abi::<D2D_MATRIX_4X4_F, Matrix4x4>();
    assert_abi::<D3DMATRIX, Matrix4x4>();
    assert_abi::<D2D_POINT_2F, Vector2>();
    assert_abi::<D2D_VECTOR_2F, Vector2>();
    assert_abi::<D2D_VECTOR_3F, Vector3>();
    assert_abi::<D2D_VECTOR_4F, Vector4>();
}

#[test]
fn native_numeric_fields_match_windows_numerics() {
    let matrix3: Matrix3x2 = unsafe {
        transmute(D2D_MATRIX_3X2_F {
            Anonymous: D2D_MATRIX_3X2_F_0 {
                m: [[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]],
            },
        })
    };
    assert_eq!(
        matrix3,
        Matrix3x2 {
            m11: 1.0,
            m12: 2.0,
            m21: 3.0,
            m22: 4.0,
            m31: 5.0,
            m32: 6.0,
        }
    );

    let values = [
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 10.0, 11.0, 12.0],
        [13.0, 14.0, 15.0, 16.0],
    ];
    let expected = Matrix4x4 {
        m11: 1.0,
        m12: 2.0,
        m13: 3.0,
        m14: 4.0,
        m21: 5.0,
        m22: 6.0,
        m23: 7.0,
        m24: 8.0,
        m31: 9.0,
        m32: 10.0,
        m33: 11.0,
        m34: 12.0,
        m41: 13.0,
        m42: 14.0,
        m43: 15.0,
        m44: 16.0,
    };
    let d2d: Matrix4x4 = unsafe {
        transmute(D2D_MATRIX_4X4_F {
            Anonymous: D2D_MATRIX_4X4_F_0 { m: values },
        })
    };
    let d3d: Matrix4x4 = unsafe {
        transmute(D3DMATRIX {
            Anonymous: D3DMATRIX_0 { m: values },
        })
    };
    assert_eq!(d2d, expected);
    assert_eq!(d3d, expected);

    let point: Vector2 = unsafe { transmute(D2D_POINT_2F { x: 1.0, y: 2.0 }) };
    let vector2: Vector2 = unsafe { transmute(D2D_VECTOR_2F { x: 1.0, y: 2.0 }) };
    let vector3: Vector3 = unsafe {
        transmute(D2D_VECTOR_3F {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        })
    };
    let vector4: Vector4 = unsafe {
        transmute(D2D_VECTOR_4F {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 4.0,
        })
    };
    assert_eq!(point, Vector2 { x: 1.0, y: 2.0 });
    assert_eq!(vector2, Vector2 { x: 1.0, y: 2.0 });
    assert_eq!(
        vector3,
        Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0
        }
    );
    assert_eq!(
        vector4,
        Vector4 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 4.0
        }
    );
}
