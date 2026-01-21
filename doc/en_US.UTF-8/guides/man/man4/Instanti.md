# Instantiate
special fileInstantiateopen a document factory[`file`] Instantiate(inmediaType template);
## DESCRIPTION


TheInstantiaterequest causes the handler to open a factory formediaTypedocuments based ontemplate. Usually this means opening an untitled buffer not bound to any file.
The handler replies when the untitled buffer is successfully opened.

Thetemplateargument
is the template of the document.
If this argument is unset
(in other words, has a value of`(char *)0`), then the template to us is in
the file named in the message's`file`attribute. If the`file`attribute is also unset, no template has been supplied, and the
factory should use its default template (usually, an empty buffer).
The data type
(mediaType) of thetemplateargument should be`string`, unless nulls are valid in the given media type,
in which case the data type must be`bytes`.
## APPLICATION USAGE


The
&cdeman.ttmedia.ptype.declare; function can be used to register for,
and help process, this message.

This message can be sent with the
&cdeman.ttmedia.load; function.
## EXAMPLES


To instantiate an appointment document, the application can send anInstantiaterequest with a first argument whose vtype is`DT_APPOINTMENT`.
## ERRORS


The ToolTalk service may return one of the following errors
in processing theInstantiaterequest:

* **TT_DESKTOP_ENOENT** 

The file that was alleged to contain the template does not exist.
* **TT_MEDIA_ERR_FORMAT** 

The template is not a valid instance of the media type.

## SEE ALSO


&cdeman.ttmedia.ptype.declare;, &cdeman.ttmedia.load;;Intro,`Deposit`,`Edit`,`Status`requests.