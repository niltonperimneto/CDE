/*
 * CDE - Common Desktop Environment
 *
 * Copyright (c) 1993-2012, The Open Group. All rights reserved.
 *
 * These libraries and programs are free software; you can
 * redistribute them and/or modify them under the terms of the GNU
 * Lesser General Public License as published by the Free Software
 * Foundation; either version 2 of the License, or (at your option)
 * any later version.
 *
 * These libraries and programs are distributed in the hope that
 * they will be useful, but WITHOUT ANY WARRANTY; without even the
 * implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
 * PURPOSE. See the GNU Lesser General Public License for more
 * details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with these libraries and programs; if not, write
 * to the Free Software Foundation, Inc., 51 Franklin Street, Fifth
 * Floor, Boston, MA 02110-1301 USA
 */
/*
 *+SNOTICE
 *
 *	$TOG: XmTextEditor.h /main/9 1998/02/03 10:30:08 mgreess $
 *
 *	RESTRICTED CONFIDENTIAL INFORMATION:
 *	
 *	The information in this document is subject to special
 *	restrictions in a confidential disclosure agreement between
 *	HP, IBM, Sun, USL, SCO and Univel.  Do not distribute this
 *	document outside HP, IBM, Sun, USL, SCO, or Univel without
 *	Sun's specific written approval.  This document and all copies
 *	and derivative works thereof must be returned or destroyed at
 *	Sun's request.
 *
 *	Copyright 1993 Sun Microsystems, Inc.  All rights reserved.
 *
 *+ENOTICE
 */

#ifndef XMTEXT_EDITOR_H
#define XMTEXT_EDITOR_H

#include "DtMailEditor.hh"
#include "Editor.hh"

class XmTextEditor : public Editor
{

  public:
    XmTextEditor(Widget parent, DtMailEditor *owner);
    ~XmTextEditor();

    virtual void	initialize(void);

    virtual void	set_contents(
				     const char *contents,
				     const unsigned long len
				     );

    virtual void	set_contents(const char * path);

    virtual char *	get_contents(void);

    virtual void	append_to_contents(
					   const char *new_contents,
					   const unsigned long len
					   );

    virtual void	append_to_contents(const char * path);

    virtual void	append_at_cursor(const char *path);
    virtual void	append_at_cursor(
				     const char *contents, 
				     const unsigned long len
				     );

    virtual void	clear_contents(void);

    virtual Widget	get_text_widget(void);

    virtual Pixel	get_text_foreground(void);
    virtual Pixel	get_text_background(void);
    virtual XmFontList	get_text_fontList(void);
    virtual Dimension   get_text_width(void);

    virtual Widget	get_editor(void);

    virtual int		get_columns(void);
    virtual int		get_rows(void);
    virtual void	set_columns(int ncolumns);
    virtual void	set_editable(Boolean value);
    virtual void	set_rows(int nrows);

    virtual void	auto_show_cursor_off(void);
    virtual void	auto_show_cursor_restore(void);
    virtual void	set_to_top(void);
    virtual void	set_to_bottom(void);

    virtual void	obtained_focus(void);
    virtual void	text_selected(void);
    virtual void	text_unselected(void);

    virtual int		no_text(void);
    
    virtual void	find_change(void);
    virtual void	spell(void);
    virtual void	format(void);
    virtual void	set_word_wrap(Boolean value);
    virtual void	undo_edit(void);
    virtual void	cut_selection(void);
    virtual void	copy_selection(void);
    virtual void	paste_from_clipboard(void);
    virtual void	paste_special_from_clipboard(Editor::InsertFormat);
    virtual void	clear_selection(void);
    virtual void	delete_selection(void);
    virtual void	select_all(void);
    virtual void	disable_redisplay(void);
    virtual void	enable_redisplay(void);

    static void MenuButtonHandler(Widget, XtPointer, XEvent *, Boolean *); 

  protected:
    // modify verify callback used during Paste Special
    static void modify_verify_callback(Widget, XtPointer, XtPointer);
    static void text_selected_callback(
				       Widget, 
				       void *, 
				       void *
				       );
    static void text_unselected_callback(
					 Widget, 
					 void *, 
					 void *
					 );
#ifdef DEAD_WOOD
    static void focus_callback(
			       Widget, 
			       void *, 
			       void *
			      );
#endif /* DEAD_WOOD */

  private:
    struct PSClientData {
	XmTextEditor *obj;
	Editor::InsertFormat insert_format;
    };
    XmTextBlockRec	*_modified_text; 
    size_t		_modified_text_buflen;
    enum PasteSpecBuf { MAXBUFSZ = 2048, BUFINC = 512};
    void modifyPasteData(XmTextVerifyCallbackStruct *, Editor::InsertFormat);
    void reallocPasteBuf(size_t size_req) {
		_modified_text->ptr = 
			(char *)realloc((void *)_modified_text->ptr, size_req);
		_modified_text_buflen = size_req; 
    }
    const char *indent_str;
    const char *begin_ins_bracket;
    const char *end_ins_bracket;
    Widget my_parent;
    Widget my_text;
    DtMailEditor *my_owner;
    Boolean text_already_selected;

    void loadFile(const char * path, const int pos);

    char *		_buffer;
    unsigned long	_buf_len;
    Boolean		_auto_show_cursor;
};

#endif // XMTEXT_EDITOR_HH
