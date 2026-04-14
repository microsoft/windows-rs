/**************************************************************************\
* 
* Copyright (c) 1998-2001, Microsoft Corp.  All Rights Reserved.
*
* Module Name:
*
*   GdiplusMatrix.h
*
* Abstract:
*
*   GDI+ Matrix class
*
\**************************************************************************/

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) /* padding added after data member */
#endif

class Matrix : public GdiplusBase
{
public:
    friend class Graphics;
    friend class GraphicsPath;
    friend class TextureBrush;
    friend class LinearGradientBrush;
    friend class PathGradientBrush;
    friend class Pen;
    friend class Region;
    
    // Default constructor is set to identity matrix.

    Matrix()
    {
        GpMatrix *matrix = NULL;

        lastResult = DllExports::GdipCreateMatrix(&matrix);
    
        SetNativeMatrix(matrix);
    }

    Matrix(IN REAL m11, 
           IN REAL m12,
           IN REAL m21, 
           IN REAL m22,
           IN REAL dx, 
           IN REAL dy)
    {
        GpMatrix *matrix = NULL;

        lastResult = DllExports::GdipCreateMatrix2(m11, m12, m21, m22, 
                                                      dx, dy, &matrix);
    
        SetNativeMatrix(matrix);
    }
    
    Matrix(IN const RectF& rect, 
           IN const PointF* dstplg)
    {
        GpMatrix *matrix = NULL;

        lastResult = DllExports::GdipCreateMatrix3(&rect, 
                                                   dstplg,
                                                   &matrix);

        SetNativeMatrix(matrix);
    }

    Matrix(IN const Rect& rect, 
           IN const Point* dstplg)
    {
        GpMatrix *matrix = NULL;

        lastResult = DllExports::GdipCreateMatrix3I(&rect, 
                                                    dstplg,
                                                    &matrix);

        SetNativeMatrix(matrix);
    }

    ~Matrix()
    {
        DllExports::GdipDeleteMatrix(nativeMatrix);
    }

    Matrix *Clone() const
    {
        GpMatrix *cloneMatrix = NULL;

        SetStatus(DllExports::GdipCloneMatrix(nativeMatrix,
                                                  &cloneMatrix));

        if (lastResult != Ok)
            return NULL;

        return new Matrix(cloneMatrix);
    }

    Status GetElements(OUT REAL *m) const 
    {
        return SetStatus(DllExports::GdipGetMatrixElements(nativeMatrix, m));
    }
    
    Status SetElements(IN REAL m11, 
                       IN REAL m12, 
                       IN REAL m21, 
                       IN REAL m22, 
                       IN REAL dx, 
                       IN REAL dy)
    {
        return SetStatus(DllExports::GdipSetMatrixElements(nativeMatrix,
                            m11, m12, m21, m22, dx, dy));
    }

    REAL OffsetX() const
    {
        REAL elements[6];

        if (GetElements(&elements[0]) == Ok)
            return elements[4];
        else 
            return 0.0f;
    }

    REAL OffsetY() const
    {
       REAL elements[6];

       if (GetElements(&elements[0]) == Ok)
           return elements[5];
       else 
           return 0.0f;
    }

    Status Reset()
    {
        // set identity matrix elements 
        return SetStatus(DllExports::GdipSetMatrixElements(nativeMatrix,
                                             1.0, 0.0, 0.0, 1.0, 0.0, 0.0));
    }

    Status Multiply(IN const Matrix *matrix, 
                    IN MatrixOrder order = MatrixOrderPrepend)
    {
        return SetStatus(DllExports::GdipMultiplyMatrix(nativeMatrix, 
                                          matrix->nativeMatrix,
                                          order));
    }

    Status Translate(IN REAL offsetX, 
                     IN REAL offsetY, 
                     IN MatrixOrder order = MatrixOrderPrepend)
    {
        return SetStatus(DllExports::GdipTranslateMatrix(nativeMatrix, offsetX,
                                                         offsetY, order));
    }

    Status Scale(IN REAL scaleX, 
                 IN REAL scaleY, 
                 IN MatrixOrder order = MatrixOrderPrepend)
    {
        return SetStatus(DllExports::GdipScaleMatrix(nativeMatrix, scaleX, 
                                                     scaleY, order));
    }

    Status Rotate(IN REAL angle, 
                  IN MatrixOrder order = MatrixOrderPrepend)
    {
        return SetStatus(DllExports::GdipRotateMatrix(nativeMatrix, angle, 
                                                      order));
    }
    
    Status RotateAt(IN REAL angle, 
                    IN const PointF& center, 
                    IN MatrixOrder order = MatrixOrderPrepend)
    {
        if(order == MatrixOrderPrepend)
        {
            SetStatus(DllExports::GdipTranslateMatrix(nativeMatrix, center.X,
                                                      center.Y, order));
            SetStatus(DllExports::GdipRotateMatrix(nativeMatrix, angle, 
                                                   order));
            return SetStatus(DllExports::GdipTranslateMatrix(nativeMatrix,
                                                             -center.X, 
                                                             -center.Y, 
                                                             order));
        }
        else
        {
            SetStatus(DllExports::GdipTranslateMatrix(nativeMatrix, 
                                                      - center.X, 
                                                      - center.Y, 
                                                      order));
            SetStatus(DllExports::GdipRotateMatrix(nativeMatrix, angle, 
                                                   order));
            return SetStatus(DllExports::GdipTranslateMatrix(nativeMatrix, 
                                                             center.X, 
                                                             center.Y, 
                                                             order));
        }
    }

    Status Shear(IN REAL shearX, 
                 IN REAL shearY,
                 IN MatrixOrder order = MatrixOrderPrepend)
    {
        return SetStatus(DllExports::GdipShearMatrix(nativeMatrix, shearX, 
                                                     shearY, order));
    }

    Status Invert()
    {
        return SetStatus(DllExports::GdipInvertMatrix(nativeMatrix));
    }

    // float version
    Status TransformPoints(IN OUT PointF* pts, 
                           IN INT count = 1) const
    {
        return SetStatus(DllExports::GdipTransformMatrixPoints(nativeMatrix, 
                                                               pts, count));
    }
    
    Status TransformPoints(IN OUT Point* pts, 
                           IN INT count = 1) const
    {
        return SetStatus(DllExports::GdipTransformMatrixPointsI(nativeMatrix, 
                                                                pts, 
                                                                count));
    }

    Status TransformVectors(IN OUT PointF* pts, 
                            IN INT count = 1) const
    { 
        return SetStatus(DllExports::GdipVectorTransformMatrixPoints(
                                        nativeMatrix, pts, count));
    }

    Status TransformVectors(IN OUT Point* pts, 
                            IN INT count = 1) const
    { 
       return SetStatus(DllExports::GdipVectorTransformMatrixPointsI(
                                        nativeMatrix, 
                                        pts, 
                                        count));
    }
    
    BOOL IsInvertible() const
    {
        BOOL result = FALSE;

        SetStatus(DllExports::GdipIsMatrixInvertible(nativeMatrix, &result));
    
        return result;
    }

    BOOL IsIdentity() const
    {
       BOOL result = FALSE;

       SetStatus(DllExports::GdipIsMatrixIdentity(nativeMatrix, &result));
    
       return result;
    }

    BOOL Equals(IN const Matrix *matrix) const
    {
       BOOL result = FALSE;

       SetStatus(DllExports::GdipIsMatrixEqual(nativeMatrix,
                                               matrix->nativeMatrix, 
                                               &result));
   
       return result;
    }
    
    Status GetLastStatus() const
    {
        Status lastStatus = lastResult;
        lastResult = Ok;
 
        return lastStatus;
    }

private:
    Matrix(const Matrix &);
    Matrix& operator=(const Matrix &);

protected:
    Matrix(GpMatrix *nativeMatrixArg)
    {
        lastResult = Ok;
        SetNativeMatrix(nativeMatrixArg);
    }
    
    VOID SetNativeMatrix(GpMatrix * nativeMatrixArg)
    {
        this->nativeMatrix = nativeMatrixArg;
    }

    Status SetStatus(Status status) const
    {
        if (status != Ok)
            return (lastResult = status);
        else
            return status;
    }

protected:
    GpMatrix *nativeMatrix;
    mutable Status lastResult;
};

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

