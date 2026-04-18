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
/* $TOG: AliasListUiItem.C /main/6 1997/09/03 17:33:31 mgreess $ */
/*
 *+SNOTICE
 *
 *      $TOG: AliasListUiItem.C /main/6 1997/09/03 17:33:31 mgreess $
 *
 *	RESTRICTED CONFIDENTIAL INFORMATION:
 *	
 *	The information in this document is subject to special
 *	restrictions in a confidential disclosure agreement bertween
 *	HP, IBM, Sun, USL, SCO and Univel.  Do not distribute this
 *	document outside HP, IBM, Sun, USL, SCO, or Univel wihtout
 *	Sun's specific written approval.  This documment and all copies
 *	and derivative works thereof must be returned or destroyed at
 *	Sun's request.
 *
 *	Copyright 1993 Sun Microsystems, Inc.  All rights reserved.
 *
 *+ENOTICE
 */

#include "RoamApp.h"
#include <Xm/List.h>
#include <DtMail/options_util.h>
#include "options_ui.h"
#include <DtMail/PropUi.hh>
#include <DtMail/ListUiItem.hh>
#include <DtMail/AliasListUiItem.hh>

extern Boolean props_changed;
void alias_stuffing_func(char * key, void * data, void * client_data);
void handleAliasSelection(Widget w, XtPointer clientdata, XtPointer calldata);

// AliasListUiItem::AliasListUiItem
// AliasListUiItem ctor
////////////////////////////////////////////////////////////////////

AliasListUiItem::AliasListUiItem(Widget w, 
		       int source, 
		       char *search_key,
		       Widget key_entry_widget,
		       Widget value_entry_widget):ListUiItem(w, source, search_key, nullptr)
{
  source = source; search_key = search_key;

  key_widget = key_entry_widget;
  value_widget = value_entry_widget;

  list_items = nullptr;
  deleted_items = nullptr;

  XtVaSetValues(w,
	XmNuserData, this,
        XmNautomaticSelection, True,
        XmNselectionPolicy, XmBROWSE_SELECT,
	nullptr);

    XtAddCallback(w,
		  XmNbrowseSelectionCallback, 
		  (XtCallbackProc)handleAliasSelection,
		  (XtPointer)this);

}

//-----------------======================-----------------
void handleAliasSelection(Widget w, XtPointer, XtPointer calldata)
{
  AliasListUiItem *item;
  XmListCallbackStruct *list_info = (XmListCallbackStruct *)calldata;
  char *selection_string = nullptr;
  DtVirtArray<PropStringPair *> *list_items;

  XtVaGetValues(w, 
	XmNuserData, &item,
	nullptr);  

  list_items = item->getItemList();

  if(list_items != nullptr)
    {                                         // motif index is 1 based
                                              //virtarry is 0 based
      PropStringPair *pair = (*list_items)[list_info->item_position - 1];

      if(pair != nullptr)
	{
	  XtVaSetValues(item->getKeyWidget(),
			XmNvalue,pair->label,
			nullptr);
	  
	  XtVaSetValues(item->getValueWidget(),
			XmNvalue,pair->value,
			nullptr);
	}
    }	
}

// AliasListUiItem::writeFromUiToSource()
// Takes values in the UI and puts them into the source DB
///////////////////////////////////////////////////////////////////
void AliasListUiItem::writeFromUiToSource()
{
  Widget w = this->getWidget();
  DtMailEnv error;
  DtMail::Session * d_session = theRoamApp.session()->session();
  DtMail::MailRc * mail_rc = d_session->mailRc(error);
  int i, num_items;

  if(deleted_items != nullptr)
    {
      num_items = deleted_items->length();

      for(i = 0; i < num_items; i++)
	{
	  mail_rc->removeAlias(error,(*deleted_items)[i]);
	}
    }

  if(list_items != nullptr)
    {
      num_items = list_items->length();
      for(i = 0; i < num_items; i++)
	{

	  mail_rc->setAlias(error,(*list_items)[i]->label,
			    (*list_items)[i]->value);
	}
    }
  
  
  if(deleted_items != nullptr)
    {
      delete deleted_items;
      deleted_items = nullptr;
    }
}

// AliasListUiItem::writeFromSourceToUi()
// Takes values in the UI and puts them into the source DB
///////////////////////////////////////////////////////////////////
static int cmp_prop_string_pair(const void *v1, const void *v2)
{
    PropStringPair	*p1 = *((PropStringPair **) v1);
    PropStringPair	*p2 = *((PropStringPair **) v2);
    int			ret;

    ret =  strcmp((const char *) p1->label, (const char *) p2->label);
    return ret;
} 
    
void AliasListUiItem::writeFromSourceToUi()
{
  DtMailEnv error;
  DtMail::Session * d_session = theRoamApp.session()->session();
  DtMail::MailRc * mail_rc = d_session->mailRc(error);
  Widget w = this->getWidget();
  const char *list_str = nullptr;
  DtVirtArray<char *> list_str_list(10);
  char *buf = nullptr;
  int list_len, i;
  const char *value = nullptr;
  PropStringPair **prop_pairs = nullptr;

  XmListDeleteAllItems(w);

  if (deleted_items != nullptr)
    {
      delete deleted_items;
      deleted_items = nullptr;
    }

  if (list_items != nullptr)
    delete list_items;

  list_items = new DtVirtArray<PropStringPair *>(10);
  mail_rc->getAliasList(alias_stuffing_func, list_items);

  list_len = list_items->length();
  if (list_len)
  {
      prop_pairs =
	(PropStringPair**) XtMalloc(list_len * sizeof(PropStringPair*));

      for (i = 0; i < list_len; i++)
      {
          prop_pairs[i] = (*list_items)[0];
	  list_items->remove(0);
      }

      qsort(
	prop_pairs,
	list_len,
	sizeof(PropStringPair*),
	cmp_prop_string_pair);

      for (i = 0; i < list_len; i++)
	list_items->append(prop_pairs[i]);

      XtFree((char*) prop_pairs);
  }

  for (i = 0; i < list_len; i++)
    list_str_list.append(
	formatPropPair( (*list_items)[i]->label, (*list_items)[i]->value ));

  options_list_init(w, &list_str_list);
}
//
// callback for creating alias list...
void alias_stuffing_func(char * key, void * data, void * client_data)
{
  DtVirtArray<PropStringPair *> *alias_list = (DtVirtArray<PropStringPair *> *)client_data;
  PropStringPair *new_pair;
  
  new_pair = new PropStringPair;

  new_pair->label = strdup(key);
  new_pair->value = strdup((char *)data);
  
  alias_list->append(new_pair);

}



///////////////////////////////////////////////////////////
void AliasListUiItem::handleAddButtonPress()
{
  char *key_str = nullptr;
  char *value_str = nullptr;
  PropStringPair *new_pair = nullptr;

  XtVaGetValues(key_widget,
		XmNvalue, &key_str,
		nullptr);

  XtVaGetValues(value_widget,
		XmNvalue, &value_str,
		nullptr);

  if(key_str != nullptr)
    if(strlen(key_str) > 0)
      {
	new_pair = new PropStringPair;
	int *pos_list, num_pos;

	new_pair->label = strdup(key_str);
	
	if(value_str != nullptr)
	  new_pair->value = strdup(value_str);
	else
	  new_pair->value = nullptr;
	
	if(XmListGetSelectedPos(this->getWidget(),
				&pos_list,
				&num_pos))
	  {
            if(list_items == nullptr)
              list_items = new DtVirtArray<PropStringPair *>(10);

	    list_items->insert(new_pair,pos_list[0] - 1); 
	
	    XmListAddItem(this->getWidget(),
			  XmStringCreateLocalized(
				       formatPropPair(
						      new_pair->label,
						      new_pair->value)),
			  pos_list[0]);
	    
	    XmListSelectPos(this->getWidget(),
			    pos_list[0],
			    TRUE);

	    XmListSetPos(this->getWidget(),
			 pos_list[0]); 
	  }	
	else
	  {
            if(list_items == nullptr)
              list_items = new DtVirtArray<PropStringPair *>(10);

	    list_items->insert(new_pair,0); 
	    XmListAddItem(this->getWidget(),
			  XmStringCreateLocalized(
				       formatPropPair(
						      new_pair->label,
						      new_pair->value)),
			  1);
	    XmListSelectPos(this->getWidget(),
			    1,
			    TRUE);

	    XmListSetPos(this->getWidget(),
			 1);	 
	  }
	  props_changed = TRUE;
      }
}
///////////////////////////////////////////////////////////
void AliasListUiItem::handleChangeButtonPress()
{
  char *key_str = nullptr;
  char *value_str = nullptr;  
  PropStringPair *new_pair = nullptr;
  XmString replace_string;
  int *pos_list, num_pos;

  // if nothing selected nothing to change...
  if(XmListGetSelectedPos(this->getWidget(),
			  &pos_list,
			  &num_pos))
    {
      XtVaGetValues(key_widget,
		    XmNvalue, &key_str,
		    nullptr);

      XtVaGetValues(value_widget,
		    XmNvalue, &value_str,
		    nullptr);

      if(key_str != nullptr)
	if(strlen(key_str) > 0)
	  {

	    new_pair = (*list_items)[pos_list[0] - 1];

	    if(deleted_items == nullptr)
	      {
		deleted_items = new DtVirtArray<char *>(10);
	      }	

	    deleted_items->append(strdup((*list_items)[pos_list[0] -1]->label));

	    free(new_pair->label);
	    new_pair->label = strdup(key_str);

	    if(new_pair->value != nullptr)
	      {
		free(new_pair->value);
		if(value_str != nullptr)
		  new_pair->value = strdup(value_str);
		else
		  new_pair->value = nullptr;
	      }

	    replace_string = XmStringCreateLocalized(
				     formatPropPair(new_pair->label,
						    new_pair->value));

	    XmListReplaceItemsPos(this->getWidget(),
				  &replace_string,
				  1,
				  pos_list[0]);

	    XmListSelectPos(this->getWidget(),
			    pos_list[0],
			    TRUE);
	  }
	props_changed = TRUE;
    }
}
///////////////////////////////////////////////////////////
void AliasListUiItem::handleDeleteButtonPress()
{
  Widget list_widget = this->getWidget();
  int *p_list, p_count;

  // get the selected position
  if(XmListGetSelectedPos(list_widget,
			  &p_list,
			  &p_count))
    {
      
      if(deleted_items == nullptr)
	{
	  deleted_items = new DtVirtArray<char *>(10);
	}	

      deleted_items->append(strdup((*list_items)[p_list[0] -1]->label));

      // delete the item from our list 
      this->list_items->remove(p_list[0] - 1); // remove only first

      // delete the item from the widget
      XmListDeletePos(list_widget, p_list[0]);

      XtVaSetValues(this->getKeyWidget(),
                        XmNvalue,"",
                        nullptr);

      XtVaSetValues(this->getValueWidget(),
                        XmNvalue,"",
                        nullptr);

      XmListSelectPos(list_widget,
		      p_list[0],
		      TRUE);

      props_changed = TRUE;
      
    }

}


