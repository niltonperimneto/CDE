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
/* $XConsortium: Test.C /main/2 1995/07/17 14:07:26 drk $ */
/*                                                                      *
 * (c) Copyright 1993, 1994 Hewlett-Packard Company                     *
 * (c) Copyright 1993, 1994 International Business Machines Corp.       *
 * (c) Copyright 1993, 1994 Sun Microsystems, Inc.                      *
 * (c) Copyright 1993, 1994 Novell, Inc.                                *
 */

#include "Application.hpp"
#include "MainWindow.hpp"
#include "MenuBar.hpp"
#include "Menu.hpp"
#include "Button.hpp"
#include "Container.hpp"
#include "IconObj.hpp"
#include "Sep.hpp"

#include <stdlib.h>
#include <stdio.h>

static void ExitCB(void *data)
{
   delete ((BaseUI *)data)->Parent();
   exit (0);
}

static void PrintUICB(void *data)
{
   ((AnyUI *) data)->DumpUIHierarchy(true);
}

static void PrintObjectsCB(void *data)
{
   ((BaseUI *) data)->DumpHierarchy();
}

class TestWindow : public MainWindow
{

 public:

   Container *container;
   
   TestWindow(MotifUI *p,char *name) : MainWindow(p,name,nullptr,"Fpprnt.l.pm") { }
   void Initialize();
};

void TestWindow::Initialize()
{
   MenuBar *mbar;
   Menu *menu;

   mbar = new MenuBar(this);
   container = new Container("PrinterSubSystem", this, "Printers");
   container->WidthHeight(600, 400);
   container->ContainerView(TREE);
   SetWorkWindow(container);

   // File menu
   menu = new Menu(mbar, "File", "F");
     new Button(menu, "Exit", PUSH_BUTTON, ExitCB, this, "x", "ALT+F4");

   // Edit menu
   menu = new Menu(mbar, "Edit", "E");
     new Button(menu, "Cut", PUSH_BUTTON, nullptr, nullptr, "t", "Shift+Del");
     new Button(menu, "Copy", PUSH_BUTTON, nullptr, nullptr, "C", "Ctrl+Ins");
     new Button(menu, "Paste", PUSH_BUTTON, nullptr, nullptr, "P", "Shift+Ins");
     new Button(menu, "Delete", PUSH_BUTTON, nullptr, nullptr, "D");

   // View menu
   menu = new Menu(mbar, "View", "V");
     new Button(menu, "New", PUSH_BUTTON, nullptr, nullptr, "N");
     new Sep(menu);
     new Button(menu, "Select All", PUSH_BUTTON, nullptr, nullptr, "S", "Ctrl+/");
     new Button(menu, "Unselect All", PUSH_BUTTON, nullptr, nullptr, "U", "Ctrl+\\");
     new Sep(menu);
     new Button(menu, "Set Preferences...", PUSH_BUTTON, nullptr, nullptr, "P");
     new Sep(menu);
     new Button(menu, "Save Settings...", PUSH_BUTTON, nullptr, nullptr, "t");

   // Actions Menu
   menu = new Menu(mbar, "Actions", "A");
     new Button(menu, "Monitor", PUSH_BUTTON, nullptr, nullptr, "M");
     new Button(menu, "Cancel Job", PUSH_BUTTON, nullptr, nullptr, "C");

   // Help Menu
   menu = new Menu(mbar, "Help", "H");
     new Button(menu, "Introduction", PUSH_BUTTON, nullptr, nullptr, "I");
     new Sep(menu);
     new Button(menu, "Tasks", PUSH_BUTTON, nullptr, nullptr, "T");
     new Button(menu, "Reference", PUSH_BUTTON, nullptr, nullptr, "R");
     new Button(menu, "On Item", PUSH_BUTTON, nullptr, nullptr, "O");
     new Sep(menu);
     new Button(menu, "Using Help", PUSH_BUTTON, nullptr, nullptr, "U");
     new Sep(menu);
     new Button(menu, "Version", PUSH_BUTTON, nullptr, nullptr, "V");
   mbar->SetHelpMenu(menu);

   // Debug menu
   menu = new Menu(mbar, "Debug", "G");
     new Button(menu, "Print Objects", PUSH_BUTTON, PrintObjectsCB, Parent());
     new Button(menu, "Print UI", PUSH_BUTTON, PrintUICB, Parent());

}

int main(int argc, 
         char **argv)
{
   Application *app = new Application("Printer", "Dtprinter", &argc, argv);
   TestWindow *window = new TestWindow(app, "Printer");
   window->Initialize();

   window->AddAction("Exit", "PrinterSubSystem", ExitCB, nullptr, "x", "Alt+F4");
   window->AddAction("Properties...", "Queue", nullptr, nullptr, "p",
		     "Ctrl+Backspace");
   window->AddSep("Queue");
   window->AddAction("Start", "Queue", nullptr, nullptr, "S");
   window->AddAction("Stop", "Queue", nullptr, nullptr, "t");
   window->AddSep("Queue");
   window->AddAction("Monitor", "Queue", nullptr, nullptr, "M");
   window->AddAction("Cancel", "PrintJob", nullptr, nullptr, "C");

   window->RegisterPopup(window->container);

   IconObj *lp0 = new IconObj("Queue", window->container, "lp0", "Fpprnt");
   window->RegisterPopup(lp0);
   IconObj *tmp = new IconObj("PrintJob", lp0, "job1", "DtPrtjb");
   window->RegisterPopup(tmp);
   tmp = new IconObj("PrintJob", lp0, "job2", "DtPrtjb");
   window->RegisterPopup(tmp);
   tmp = new IconObj("PrintJob", lp0, "job3", "DtPrtjb");
   window->RegisterPopup(tmp);
   IconObj *lp1 = new IconObj("Queue", window->container, "lp1", "Fpprnt");
   window->RegisterPopup(lp1);
   tmp = new IconObj("PrintJob", lp1, "job4", "DtPrtjb");
   window->RegisterPopup(tmp);
   tmp = new IconObj("PrintJob", lp1, "job5", "DtPrtjb");
   window->RegisterPopup(tmp);

   app->Visible(true);
   app->Run();

   return 0;
}
