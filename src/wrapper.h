#pragma once
#include "rust/cxx.h"
#include "orbitersdk.h"
#include <cstdint>

using oapi::Brush;
using oapi::Font;
using oapi::IVECTOR2;
using oapi::Pen;
using std::unique_ptr;

void debugLog(rust::Str);

static int MsgProc(UINT msg, UINT mfd, WPARAM wparam, LPARAM lparam);

struct OapiSketchpad
{
    oapi::Sketchpad* pad_;

    OapiSketchpad(oapi::Sketchpad* pad) : pad_(pad) {}
    /**
	 * \brief Selects a new font to use.
	 * \param font pointer to font resource
	 * \return Previously selected font.
	 * \default None, returns NULL.
	 * \sa oapi::Font, oapi::GraphicsClient::clbkCreateFont
	 */
    void SetFont(const Font& font) const
    {
        pad_->SetFont((Font*)&font);
    }

    /**
	 * \brief Selects a new pen to use.
	 * \param pen pointer to pen resource, or NULL to disable outlines
	 * \return Previously selected pen.
	 * \default None, returns NULL.
	 * \sa oapi::Pen, oapi::GraphicsClient::clbkCreatePen
	 */
     Pen *SetPen(Pen *pen) const { return pad_->SetPen(pen); }

    /**
	 * \brief Selects a new brush to use.
	 * \param brush pointer to brush resource, or NULL to disable fill mode
	 * \return Previously selected brush.
	 * \default None, returns NULL.
	 * \sa oapi::Brush, oapi::GraphicsClient::clbkCreateBrush
	 */
     Brush *SetBrush(Brush *brush) const { return pad_->SetBrush(brush); }

     /**
	 * \brief Set horizontal and vertical text alignment.
	 * \param tah horizontal alignment
	 * \param tav vertical alignment
	 * \default None.
	 */
     void SetTextAlign(oapi::Sketchpad::TAlign_horizontal tah = oapi::Sketchpad::TAlign_horizontal::LEFT, oapi::Sketchpad::TAlign_vertical tav = oapi::Sketchpad::TAlign_vertical::TOP) { pad_->SetTextAlign(tah, tav); }

     /**
	 * \brief Set the foreground colour for text output.
	 * \param col colour description (format: 0xBBGGRR)
	 * \return Previous colour setting.
	 * \default None, returns 0.
	 */
     uint32_t SetTextColor(uint32_t col) { return pad_->SetTextColor(col); }

     /**
	 * \brief Set the background colour for text output.
	 * \param col background colour description (format: 0xBBGGRR)
	 * \return Previous colour setting
	 * \default None, returns 0.
	 * \note The background colour is only used if the background mode
	 *   is set to BK_OPAQUE.
	 * \sa SetBackgroundMode
	 */
     uint32_t SetBackgroundColor(uint32_t col) { return pad_->SetBackgroundColor(col); }


     /**
	 * \brief Set the background mode for text and drawing operations.
	 * \param mode background mode (see \ref BkgMode)
	 * \default None.
	 * \note This function affects text output and dashed line drawing.
	 * \note In opaque background mode, text background and the gaps
	 *   between dashed lines are drawn in the current background colour
	 *   (see SetBackgroundColor). In transparent mode, text background
	 *   and line gaps are not modified.
	 * \note The default background mode (before the first call of
	 *   SetBackgroundMode) should be transparent.
	 * \sa SetBackgroundColor, SetTextColor
	 */
     void SetBackgroundMode(oapi::Sketchpad::BkgMode mode) { pad_->SetBackgroundMode(mode); }

     /**
	 * \brief Return height and (average) width of a character in the currently
	 *   selected font.
	 * \return Height of character cell [pixel] in the lower 16 bit of the return value,
	 *   and (average) width of character cell [pixel] in the upper 16 bit.
	 * \default None, returns 0.
	 * \note The height value should describe the height of the character cell (i.e.
	 *   the smallest box circumscribing all characters in the font), but without any
	 *   "internal leading", i.e. the gap between characters in two consecutive lines.
	 * \note For proportional fonts, the width value should be an approximate average
	 *   character width.
	 */
     uint32_t GetCharSize() { return pad_->GetCharSize(); }

     /**
	 * \brief Return the width of a text string in the currently selected font.
	 * \param str text string
	 * \param len string length, or 0 for auto (0-terminated string)
	 * \return width of the string, drawn in the currently selected font [pixel]
	 * \default None, returns 0.
	 * \sa SetFont
	 */
     uint32_t GetTextWidth(rust::Str str) { return pad_->GetTextWidth(str.data(), str.length()); }

     /**
	 * \brief Set the position in the surface bitmap which is mapped to the
	 *   origin of the coordinate system for all drawing functions.
	 * \param x horizontal position of the origin [pixel]
	 * \param y vertical position of the origin [pixel]
	 * \default None.
	 * \note By default, the reference point for drawing function coordinates is
	 *   the top left corner of the bitmap, with positive x-axis to the right,
	 *   and positive y-axis down.
	 * \note SetOrigin can be used to shift the logical reference point to a
	 *   different position in the surface bitmap (but not to change the
	 *   orientation of the axes).
	 * \note If the drawing system used by an implementation does not support
	 *   this function directly, the derived class should itself account for the
	 *   shift in origin, by subtracting the offset from all coordinate values.
	 * \sa GetOrigin
	 */
     void SetOrigin(int x, int y) { pad_->SetOrigin(x, y); }

     /**
	 * \brief Returns the position in the surface bitmap which is mapped to
	 *   the origin of the coordinate system for all drawing functions.
	 * \param [out] x pointer to integer receiving horizontal position of the origin [pixel]
	 * \param [out] y pointer to integer receiving vertical position of the origin [pixel]
	 * \default Returns (0,0)
	 * \sa SetOrigin
	 */
     void GetOrigin(int *x, int *y) const { pad_->GetOrigin(x, y); }

     /**
	 * \brief Draw a text string.
	 * \param x reference x position [pixel]
	 * \param y reference y position [pixel]
	 * \param str text string
	 * \param len string length for output
	 * \return \e true on success, \e false on failure.
	 * \default None, returns false.
	 */
     bool Text(int x, int y, rust::Str str) { return pad_->Text(x, y, str.data(), str.length()); }

     /**
	 * \brief Draw a text string into a rectangle.
	 * \param x1 left edge [pixel]
	 * \param y1 top edge [pixel]
	 * \param x2 right edge [pixel]
	 * \param y2 bottom edge [pixel]
	 * \param str text string
	 * \param len string length for output
	 * \return \e true on success, \e false on failure.
	 * \default Implementation via \ref Text calls.
	 * \note This method should write the text string into the specified
	 *   rectangle, using the current font. Line breaks should automatically
	 *   be applied as required to fit the text in the box.
	 * \note The bottom edge (y2) should probably be ignored, so text isn't
	 *   truncated if it doesn't fit the box.
	 */
     bool TextBox(int x1, int y1, int x2, int y2, const char *str, int len) {
        pad_->TextBox(x1, y1, x2, y2, str, len);
     }

    /**
	 * \brief Draw a single pixel in a specified colour.
	 * \param x x-coordinate of point [pixel]
	 * \param y y-coordinate of point [pixel]
	 * \param col pixel colour (format: 0xBBGGRR)
	 */
     void Pixel(int x, int y, uint32_t col) { pad_-> Pixel(x, y, col); }

     /**
	 * \brief Move the drawing reference to a new point.
	 * \param x x-coordinate of new reference point [pixel]
	 * \param y y-coordinate of new reference point [pixel]
	 * \note Some methods use the drawing reference point for
	 *   drawing operations, e.g. \ref LineTo.
	 * \default None.
	 * \sa LineTo
	 */
     void MoveTo(int x, int y) { pad_->MoveTo(x, y); }

     /**
	 * \brief Draw a line to a specified point.
	 * \param x x-coordinate of line end point [pixel]
	 * \param y y-coordinate of line end point [pixel]
	 * \default None.
	 * \note The line starts at the current drawing reference
	 *   point.
	 * \sa MoveTo
	 */
     void LineTo(int x, int y) { pad_->LineTo(x, y); }

     /**
	 * \brief Draw a line between two points.
	 * \param x0 x-coordinate of first point [pixel]
	 * \param y0 y-coordinate of first point [pixel]
	 * \param x1 x-coordinate of second point [pixel]
	 * \param y1 y-coordinate of second point [pixel]
	 * \default None.
	 * \note The line is drawn with the currently selected pen.
	 * \sa SetPen
	 */
     void Line(int x0, int y0, int x1, int y1) { pad_-> Line(x0, y0, x1, y1); }

     /**
	 * \brief Draw a rectangle (filled or outline).
	 * \param x0 left edge of rectangle [pixel]
	 * \param y0 top edge of rectangle [pixel]
	 * \param x1 right edge of rectangle [pixel]
	 * \param y1 bottom edge of rectangle [pixel]
	 * \default Draws the rectangle from 4 line segments by
	 *   calling \ref MoveTo and \ref LineTo.
	 * \note Derived classes should overload this method if possible,
	 *   because the default method does not allow to draw filled
	 *   rectangles, and may be less efficient than a dedicated
	 *   implementation.
	 * \note Implementations should fill the rectangle with the
	 *   currently selected brush resource.
	 * \sa MoveTo, LineTo, Ellipse, Polygon
	 */
     void Rectangle(int x0, int y0, int x1, int y1) { pad_-> Rectangle(x0, y0, x1, y1); }

     /**
	 * \brief Draw an ellipse from its bounding box.
	 * \param x0 left edge of bounding box [pixel]
	 * \param y0 top edge of bounding box [pixel]
	 * \param x1 right edge of bounding box [pixel]
	 * \param y1 bottom edge of bounding box [pixel]
	 * \default None.
	 * \note Implementations should fill the ellipse with the
	 *   currently selected brush resource.
	 * \sa Rectangle, Polygon
	 */
     void Ellipse(int x0, int y0, int x1, int y1) { pad_->Ellipse(x0, y0, x1, y1); }

     /**
	 * \brief Draw a closed polygon given by vertex points.
	 * \param pt list of vertex points
	 * \param npt number of points in the list
	 * \default None.
	 * \note Implementations should draw the outline of the 
	 *   polygon with the current pen, and fill it with the
	 *   current brush.
	 * \note The polygon should be closed, i.e. the last point
	 *   joined with the first one.
	 * \sa Polyline, PolyPolygon, Rectangle, Ellipse
	 */
     void Polygon(const IVECTOR2 *pt, int npt) { pad_->Polygon(pt, npt); }

     /**
	 * \brief Draw a line of piecewise straight segments.
	 * \param pt list of vertex points
	 * \param npt number of points in the list
	 * \default None
	 * \note Implementations should draw the line with the
	 *   currently selected pen.
	 * \note Polylines are open figures: the end points are
	 *   not connected, and no fill operation is performed.
	 * \sa Polygon, PolyPolyline, Rectangle, Ellipse
	 */
     void Polyline(const IVECTOR2 *pt, int npt) { pad_->Polyline(pt, npt); }

    /**
	 * \brief Draw a set of polygons.
	 * \param pt list of vertex points for all polygons
	 * \param npt list of number of points for each polygon
	 * \param nline number of polygons
	 * \default Calls Polygon for each line in the list.
	 * \note The number of entries in npt must be >= nline, and
	 *   the number of points in pt must be at least the sum of
	 *   the values in npt.
	 * \note Implementations should overload this function if
	 *   they can provide efficient direct support for it. Otherwise,
	 *   the base class implementation should be sufficient.
	 * \sa Polygon, Polyline, PolyPolyline
	 */
     void PolyPolygon(const IVECTOR2 *pt, const int *npt, const int nline) { pad_-> PolyPolygon(pt, npt, nline); }

    /**
	 * \brief Draw a set of polylines.
	 * \param pt list of vertex points for all lines
	 * \param npt list of number of points for each line
	 * \param nline number of lines
	 * \default Calls Polyline for each line in the list.
	 * \note The number of entries in npt must be >= nline, and
	 *   the number of points in pt must be at least the sum of
	 *   the values in npt.
	 * \note Implementations should overload this function if
	 *   they can provide efficient direct support for it. Otherwise,
	 *   the base class implementation should be sufficient.
	 * \sa Polyline, Polygon, PolyPolygon
	 */
     void PolyPolyline(const IVECTOR2 *pt, const int *npt, const int nline) { pad_->PolyPolyline(pt, npt, nline); }

     /**
	 * \brief Returns the surface associated with the drawing object.
	 * \return Surface handle
	 */
     inline SURFHANDLE GetSurface() const { return pad_->GetSurface(); }

     /**
	 * \brief Return the Windows device context handle, if applicable.
	 * \return device context handle
	 * \default None, returns NULL.
	 * \note Sketchpad implementations based on the Windows GDI system
	 *   should overload this function to return the device context handle
	 *   here. All other implementations should not overload this function.
	 * \note The device context returned by this function should not be
	 *   released (e.g. with ReleaseDC). The device context is released
	 *   automatically when the Sketchpad instance is destroyed.
	 * \note This method should be regarded as temporary. Ultimately, the
	 *   device-dependent drawing mechanism should be hidden outside the
	 *   sketchpad implementation.
	 */
     HDC GetDC() { return pad_->GetDC(); }
};
#include "src/lib.rs.h"

extern "C" RustMFD *create_rust_mfd() noexcept;

void InitModuleSpec(rust::Str name, unsigned int key);
void ExitModuleSpec();

class MFDTemplate : public MFD2
{
public:
	MFDTemplate(DWORD w, DWORD h, VESSEL *vessel);
	~MFDTemplate();
	char *ButtonLabel(int bt);
	int ButtonMenu(const MFDBUTTONMENU **menu) const;
	bool ConsumeButton(int bt, int event);
	bool Update(oapi::Sketchpad *skp);

protected:
	oapi::Font *font;
	RustMFD* rust_mfd_;
};
