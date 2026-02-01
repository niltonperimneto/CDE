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
 *	$XConsortium: CAPIMethods.C /main/4 1996/04/21 19:47:13 drk $
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

#include <DtMail/DtMail.h>
#include <DtMail/DtMail.hh>

extern "C" DtMailSession
DtMailCreateSession(DtMailEnv * error,
		    const char * app_name)
{
  DtMailEnv & err = *error;
  DtMail::Session	* session = new DtMail::Session(err, app_name);

  return((DtMailSession)session);
}

extern "C" void
DtMailCloseSession(DtMailEnv * error,
		   DtMailSession session,
		   DtMailBoolean)
{
  DtMailEnv & err = *error;
  DtMail::Session	* _session = (DtMail::Session *)session;

  err.clear();

  delete _session;
}

extern "C" int
DtMailGetMinorCode(DtMailEnv * error)
{
  DtMailEnv & err = *error;
  return(DtMail::getMinorCode(err));
}

extern "C" const char *
DtMailGetErrorString(DtMailEnv * error)
{
  DtMailEnv & err = *error;
  return(DtMail::getErrorString(err));
}

extern "C" const char **
DtMailEnumerateImpls(DtMailSession session, DtMailEnv * error)
{
  DtMailEnv & err = *error;
  DtMail::Session	* _session = (DtMail::Session *)session;

  return(_session->enumerateImpls(err));
}

extern "C" void
DtMailSetDefaultImpl(DtMailSession session,
		     DtMailEnv * error,
		     const char * impl)
{
  DtMailEnv & err = *error;
  DtMail::Session	* _session = (DtMail::Session *)session;

  _session->setDefaultImpl(err, impl);
  return;
}

extern "C" const char *
DtMailGetDefaultImpl(DtMailSession session, DtMailEnv * error)
{
  DtMailEnv & err = *error;
  DtMail::Session	* _session = (DtMail::Session *)session;

  return(_session->getDefaultImpl(err));
}

extern "C" void
DtMailQueryImpl(DtMailSession session,
		DtMailEnv * error,
		const char * impl,
		const char * capability,
		...)
{
  DtMailEnv & err = *error;
  va_list		args;
  DtMail::Session	* _session = (DtMail::Session *)session;

  va_start(args, capability);
  _session->queryImplV(err, impl, capability, args);
  va_end(args);
}

extern "C" DtMailMailBox
DtMailMailBoxConstruct(DtMailSession session,
		       DtMailEnv * error,
		       DtMailObjectSpace space,
		       void * path,
		       DtMailCallback cb_func,
		       void * client_data)
{
  DtMailEnv & err = *error;
  DtMail::Session	* _session = (DtMail::Session *)session;

  return((DtMailMailBox)_session->mailBoxConstruct(err,
						   space,
						   path,
						   cb_func,
						   client_data));
}

extern "C" void
DtMailMailBoxDestruct(DtMailMailBox mbox)
{
  DtMail::MailBox *mb = (DtMail::MailBox *)mbox;

  delete mb;
}

extern "C" void
DtMailMailBoxCreate(DtMailMailBox mbox, DtMailEnv * error)
{
  DtMailEnv & err = *error;
  DtMail::MailBox	*mb = (DtMail::MailBox *)mbox;

  mb->create(err);
}

extern "C" void
DtMailMailBoxOpen(DtMailMailBox mbox, DtMailEnv * error, DtMailBoolean auto_create)
{
  DtMailEnv & err = *error;
  DtMail::MailBox 	*mb = (DtMail::MailBox *)mbox;

  mb->open(err, auto_create);
}

extern "C" DtMailMessage
DtMailMailBoxGetFirstMessage(DtMailMailBox mbox, DtMailEnv * error)
{
  DtMailEnv & err = *error;
  DtMail::MailBox	*mb = (DtMail::MailBox *)mbox;

  return((DtMailMessage)mb->getFirstMessage(err));
}

extern "C" DtMailMessage
DtMailMailBoxGetNextMessage(DtMailMailBox mbox, DtMailEnv * error, DtMailMessage last)
{
  DtMailEnv & err = *error;
  DtMail::MailBox	*mb = (DtMail::MailBox *)mbox;
  DtMail::Message 	*ml = (DtMail::Message *)last;

  return((DtMailMessage)mb->getNextMessage(err, ml));
}

extern "C" DtMailMessage
DtMailMailBoxNewMessage(DtMailMailBox mbox, DtMailEnv * error)
{
  DtMailEnv & err = *error;
  DtMail::MailBox	*mb = (DtMail::MailBox *)mbox;

  return((DtMailMessage)mb->newMessage(err));
}

/*

extern "C" DtMailHeaderHandle
DtMailEnvelopeGetFirstHeader(DtMailEnvelope ev,
			     DtMailEnv * error,
			     char ** name,
			     DtMailValueSeq ** value,
			     DtMailBoolean translate_name)
{
  DtMailEnv & err = *error;
  DtMail::Envelope 	*env = (DtMail::Envelope *)ev;

  return(env->getFirstHeader(*err, name, value, translate_name));
}

extern "C" DtMailHeaderHandle
DtMailEnvelopeGetNextHeader(DtMailEnvelope ev,
			    DtMailEnv * error,
			    DtMailHeaderHandle last,
			    char ** name,
			    DtMailValueSeq ** value,
			    DtMailBoolean translate_name)
{
  DtMailEnv & err = *error;
  DtMail::Envelope	*env = (DtMail::Envelope *)ev;

  return(env->getNextHeader(*err, last, name, value, translate_name));
}

extern "C" void
DtMailEnvelopeGetHeader(DtMailEnvelope ev,
			    DtMailEnv * error,
			    const const char * name,
			    DtMailValueSeq ** value)
{
  DtMailEnv & err = *error;
  DtMail::Envelope	*env = (DtMail::Envelope *)ev;

  env->getHeader(*err, name, value);
}

extern "C" void
DtMailEnvelopeSetHeaderSeq(DtMailEnvelope ev,
			   DtMailEnv * error,
			   const const char * name,
			   const DtMailValueSeq * value)
{
  DtMailEnv & err = *error;
  DtMail::Envelope	*env = (DtMail::Envelope *)ev;

  env->setHeaderSeq(*err, name, value);
}

extern "C" void
DtMailEnvelopeSetHeader(DtMailEnvelope ev,
			DtMailEnv * error,
			const const char * name,
			const DtMailValue * value,
			DtMailBoolean replace)
{
  DtMailEnv & err = *error;
  DtMail::Envelope	*env = (DtMail::Envelope *)ev;

  env->setHeader(*err, name, value, replace);
}

extern "C" void
DtMailEnvelopeTransportName(DtMailEnvelope ev,
			    DtMailEnv * error,
			    const char * name,
			    char ** trans_name)
{
  DtMailEnv & err = *error;
  DtMail::Envelope 	*env = (DtMail::Envelope *)ev;

  env->transportName(*err, name, trans_name);
}

extern "C" void *
DtMailEnvelopeObject(DtMailEnvelope ev, DtMailEnv * error)
{
  DtMailEnv & err = *error;
  DtMail::Envelope 	*env = (DtMail::Envelope *)ev;

  return(env->object(err));
}

extern "C" void
DtMailEnvelopeDestroy(DtMailEnvelope ev)
{
  DtMail::Envelope 	*env = (DtMail::Envelope *)ev;

  delete env;
  return;
}

extern "C" DtMailHeaderHandle
DtMailBodyPartGetFirstHeader(DtMailBodyPart bp, 
			     DtMailEnv * error, 
			     char ** name, 
			     DtMailValueSeq ** value,
			     DtMailBoolean trans)
{
  DtMailEnv & err = *error;
  DtMail::BodyPart 	*bpp = (DtMail::BodyPart *)bp;

  return(bpp->getFirstHeader(*err, name, value, trans));
}

extern "C" DtMailHeaderHandle
DtMailBodyPartGetNextHeader(DtMailBodyPart bp, 
			    DtMailEnv * error, 
			    DtMailHeaderHandle hd, 
			    char ** name,
			    DtMailValueSeq ** value, 
			    DtMailBoolean trans)
{
  DtMailEnv & err = *error;
  DtMail::BodyPart 	*bpp = (DtMail::BodyPart *)bp;

  return(bpp->getNextHeader(*err, hd, name, value, trans));
}

extern "C" void
DtMailBodyPartGetHeader(DtMailBodyPart bp, 
			DtMailEnv * error, 
			const const char * name, 
			DtMailValueSeq ** value)
{
  DtMailEnv & err = *error;
  DtMail::BodyPart 	*bpp = (DtMail::BodyPart *)bp;

  bpp->getHeader(*err, name, value);
}

extern "C" void
DtMailBodyPartSetHeaderSeq(DtMailBodyPart bp, 
			   DtMailEnv * error, 
			   const const char * name, 
			   const DtMailValueSeq * value)
{
  DtMailEnv & err = *error;
  DtMail::BodyPart 	*bpp = (DtMail::BodyPart *)bp;

  bpp->setHeaderSeq(*err, name, value);
}

extern "C" void
DtMailBodyPartSetHeader(DtMailBodyPart bp, 
			DtMailEnv * error, 
			const const char * name, 
			const DtMailValue * value,
			DtMailBoolean replace)
{
  DtMailEnv & err = *error;
  DtMail::BodyPart 	*bpp = (DtMail::BodyPart *)bp;

  bpp->setHeader(*err, name, value, replace);
}

extern "C" void
DtMailBodyPartTransportName(DtMailBodyPart bp, 
			    DtMailEnv * error, 
			    const char * name, 
			    char ** trans_name)
{
  DtMailEnv & err = *error;
  DtMail::BodyPart *bpp = (DtMail::BodyPart *)bp;

  bpp->transportName(*err, name, trans_name);
}

extern "C" void *
DtMailBodyPartObject(DtMailBodyPart bp, DtMailEnv * error)
{
  DtMailEnv & err = *error;
  DtMail::BodyPart *bpp = (DtMail::BodyPart *)bp;

  return(bpp->object(err));
}

extern "C" DtMailBoolean
DtMailBodyPartIsContainer(DtMailBodyPart bp, DtMailEnv *error)
{
  DtMailEnv & err = *error;
  DtMail::BodyPart *bpp = (DtMail::BodyPart *)bp;

  return(bpp->isContainer(err));
}

extern "C" DtMailMessage *
DtMailBodyPartGetEmbeddedMessage(DtMailBodyPart bp, DtMailEnv *error)
{
  DtMailEnv & err = *error;
  DtMail::BodyPart *bpp = (DtMail::BodyPart *)bp;

  return((DtMailMessage *)bpp->getEmbeddedMessage(err));
}

extern "C" void
DtMailBodyPartGetContents(DtMailBodyPart bp, DtMailEnv * error, void ** contents, unsigned long * length,
		       const char * * type)
{
  DtMailEnv & err = *error;
  DtMail::BodyPart *bpp = (DtMail::BodyPart *)bp;

  bpp->getContents(*err, contents, *length, type);
}

extern "C" void
DtMailBodyPartSetContents(DtMailBodyPart bp, DtMailEnv * error, const void * contents, const unsigned long length,
		       const const char * type)
{
  DtMailEnv & err = *error;
  DtMail::BodyPart *bpp = (DtMail::BodyPart *)bp;

  bpp->setContents(*err, contents, length, type);
}

extern "C" void
DtMailBodyPartDestroy(DtMailBodyPart bp)
{
  DtMail::BodyPart *bpp = (DtMail::BodyPart *)bp;

  delete bpp;
}

extern "C" DtMailMessage
DtMailFileMessageConstruct(DtMailEnv * error,
			   DtMailSession session,
			   const char * path,
			   FileCallback cb_func,
			   void * client_data)
{
  DtMailEnv & err = *error;

  return((DtMailMessage)new DtMail::FileMessage(*err, 
						session, 
						path, 
						cb_func, 
						client_data));
}

extern "C" void
DtMailFileMessageDestruct(DtMailMessage msg)
{
  DtMail::FileMessage * fm = (DtMail::FileMessage *)msg;

    delete fm;
}

extern "C" DtMailBoolean
DtMailFileMessageIsModified(DtMailMessage msg, DtMailEnv * error)
{
  DtMailEnv & err = *error;
  DtMail::FileMessage * fm = (DtMail::FileMessage *)msg;

  return(fm->isModified(err));
}

extern "C" void
DtMailFileMessageRequestSave(DtMailMessage msg, DtMailEnv * error)
{
  DtMailEnv & err = *error;
  DtMail::FileMessage * fm = (DtMail::FileMessage *)msg;

  fm->requestSave(err);
}

extern "C" void
DtMailFileMessageRequestRevert(DtMailMessage msg, DtMailEnv * error)
{
  DtMailEnv & err = *error;
  DtMail::FileMessage * fm = (DtMail::FileMessage *)msg;

  fm->requestRevert(err);
}

extern "C" void
DtMailFileMessageRequestOpen(DtMailMessage msg, DtMailEnv * error)
{
  DtMailEnv & err = *error;
  DtMail::FileMessage * fm = (DtMail::FileMessage *)msg;

  fm->requestOpen(err);
}

extern "C" void
DtMailFileMessageCreate(DtMailMessage msg, DtMailEnv * error)
{
  DtMailEnv & err = *error;
  DtMail::FileMessage * fm = (DtMail::FileMessage *)msg;

  fm->create(err);
}

extern "C" void
DtMailFileMessageOpen(DtMailMessage msg, DtMailEnv * error, DtMailBoolean auto_create)
{
  DtMailEnv & err = *error;
  DtMail::FileMessage * fm = (DtMail::FileMessage *)msg;

  fm->open(*err, auto_create);
}

extern "C" DtMailMessage
DtMailEmbeddedMessageCreate(DtMailEnv * error, DtMailMailBox owner)
{
  DtMailEnv & err = *error;
  DtMail::MailBox * mb = (DtMail::MailBox *)owner;

  return((DtMailMessage)new DtMail::EmbeddedMessage(*err, *mb));
}

extern "C" void
DtMailEmbeddedMessageDestroy(DtMailMessage msg)
{
  DtMailEnv & err = *error;
  DtMail::EmbeddedMessage * em = (DtMail::EmbeddedMessage *)msg;

  delete em;
}

extern "C" DtMailEnvelope
DtMailMessageGetEnvelope(DtMailMessage bmm, DtMailEnv * error)
{
  DtMailEnv & err = *error;
  DtMail::Message * msg = (DtMail::Message *)bmm;

  return((DtMailEnvelope)msg->getEnvelope(err));
}

extern "C" int
DtMailMessageGetBodyCount(DtMailMessage bmm, DtMailEnv * error)
{
  DtMailEnv & err = *error;
  DtMail::Message * msg = (DtMail::Message *)bmm;

  return(msg->getBodyCount(err));
}

extern "C" DtMailBodyPart *
DtMailMessageGetFirstBodyPart(DtMailMessage bmm, DtMailEnv * error)
{
  DtMailEnv & err = *error;
  DtMail::Message * msg = (DtMail::Message *)bmm;

  return((DtMailBodyPart *)msg->getFirstBodyPart(err));
}

extern "C" DtMailBodyPart *
DtMailMessageGetNextBodyPart(DtMailMessage bmm, DtMailEnv * error, DtMailBodyPart * bml)
{
  DtMailEnv & err = *error;
  DtMail::Message * msg = (DtMail::Message *)bmm;
  DtMail::BodyPart * last = (DtMail::BodyPart *)bml;

  return((DtMailBodyPart *)msg->getNextBodyPart(*err, last));
}

extern "C" DtMailBodyPart *
DtMailMessageNewBodyPart(DtMailMessage bmm, DtMailEnv * error)
{
  DtMailEnv & err = *error;
  DtMail::Message * msg = (DtMail::Message *)bmm;

  return((DtMailBodyPart *)msg->newBodyPart(err));
}

extern "C" DtMailMessage
DtMailMessageContainer(DtMailMessage bmm, DtMailEnv * error)
{
  DtMailEnv & err = *error;
  DtMail::Message * msg = (DtMail::Message *)bmm;

  return(msg->message(err));
}

*/

