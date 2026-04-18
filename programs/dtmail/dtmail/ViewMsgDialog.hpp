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
 *	$TOG: ViewMsgDialog.h /main/5 1998/01/28 18:36:00 mgreess $
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

#ifndef VIEWMSGDIALOG_H
#define VIEWMSGDIALOG_H

#include <DtMail/DtMail.hh>
#include "DialogShell.hpp"
#include "Editor.hh"
#include "Dialog.hpp"
#include "AttachArea.hpp"
#include "DtMailEditor.hh"
#include "MenuWindow.hpp"
#include "MenuBar.hpp"
#include "DtMailGenDialog.hh"
#include "NoOpCmd.hpp"
#include "QuitCmd.hpp"
#include "UndoCmd.hpp"
#include "IconifyCmd.hpp"
#include "CmdList.hpp"
#include "RoamCmds.hpp"
#include <DtMail/DtMailError.hh>


// Forward declarations

class Cmd;
class RoamMenuWindow;


class ViewMsgDialog : public DialogShell, public AbstractEditorParent {

  public:
    
    ViewMsgDialog(RoamMenuWindow*, WidgetClass wc=xmDialogShellWidgetClass);
    virtual ~ViewMsgDialog();
    
    virtual void initialize(void);
    virtual void quit(void);

    virtual const char *const className () { return "ViewMsg"; }
    void propsChanged(void);

// Accessors
    

//  AttachArea	*attachArea(){ return _attach; }
    DtMailMessageHandle msgno() { return _msgno; }
    RoamMenuWindow *parent() { return _parent; };	 
    void attachmentFeedback(Boolean);   

// Mutators
    void msgno( DtMailMessageHandle msg_num ) { _msgno=msg_num; }
    void append(const char *, const unsigned long);
    void text(const char *, const unsigned long);
    
    void popped_down(void);
    void popped_up(void);
    
// SR - Added methods below

    void raise(void);
    DtMailEditor* get_editor(void);

    DtMailGenDialog *genDialog() { return _genDialog; }

    void    auto_show_cursor_off(void);
    void    auto_show_cursor_restore(void);
    void    set_to_top(void);

  // SR - Text-selection callbacks.

    virtual void text_selected(void);
    virtual void text_unselected(void);

    void	attachment_selected(void);
    void	all_attachments_deselected(void);
    void	all_attachments_selected(void);
    void	selectAllAttachments(void);
    void	add_att(char *) { ; }
    void	add_att(char *, DtMailBuffer) { ; }
    void	add_att(DtMailBuffer) { ; }

    void	save_selected_attachment(char *);	

    void	showAttachArea(void);
    void	hideAttachArea(void);

    void	activate_default_attach_menu(void);
    void	deactivate_default_attach_menu(void);

    void	addAttachmentActions(
				    char **,
				    int
		);
    void	removeAttachmentActions(void);
    void	invokeAttachmentAction(int);	  	

    virtual void manage(void);

    static void save_attachment_callback( void *, char *);    

  protected:

    void createMenuPanes(void);
    Widget createWorkArea( Widget );  
  
    void	construct_edit_menu(void);
    void	construct_attachment_menu(void);
    void	construct_attachment_popup(void);
    void	construct_text_popup(void);
    void  construct_help_menu(void);

  private:

    RoamMenuWindow *_parent;	
    Widget _workArea;
    Widget _main;

    MenuBar *_menuBar;
    DtMailGenDialog *_genDialog;

    Cmd*  _edit_copy;
    Cmd*  _edit_select_all;

    // Attachment
    Widget   _attachmentMenu;
    CmdList *_attachmentActionsList;
    CmdList *_attachmentMenuList;
    CmdList *_attachmentPopupMenuList;
    CmdList *_textPopupMenuList;
    Cmd*  _attach_save_as;
    Cmd*  _attach_select_all;

    // Help Menu
    Cmd *_overview;
    Cmd *_tasks;
    Cmd *_reference;
    Cmd *_on_item;
    Cmd *_using_help;
    Cmd *_about_mailer;

    Widget _text;
    DtMailEditor *my_editor;
    DtMailMessageHandle	_msgno;
};

#endif
