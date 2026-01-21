# csa/csa.h
file formatscsa/csa.hcalendar and appointment services definitions#include <csa/csa.h>
## DESCRIPTION


Thecsa/csa.hheader defines the following data types through`typedef`:typedef short int               CSA_sint16;
typedef long int                CSA_sint32;
typedef unsigned char           CSA_uint8;
typedef unsigned short int      CSA_uint16;
typedef unsigned long int       CSA_uint32;
typedef void *                  CSA_buffer;
typedef CSA_uint32              CSA_entry_handle;
typedef CSA_uint32              CSA_session_handle;
typedef char *                  CSA_string;
typedef CSA_string      CSA_attribute_reference;
typedef CSA_uint32      CSA_boolean;
typedef CSA_string      CSA_date_time;
typedef CSA_string      CSA_date_time_range;
typedef CSA_sint32      CSA_enum;
typedef CSA_uint32      CSA_flags;
typedef CSA_uint32      CSA_return_code;
typedef CSA_string      CSA_service_reference;
typedef CSA_string      CSA_time_duration;
#define CSA_FALSE               ((CSA_boolean)0)
#define CSA_TRUE                ((CSA_boolean)1)
/* EXTENSION FLAGS */
#define CSA_EXT_REQUIRED        ((CSA_flags)0x10000)
#define CSA_EXT_OUTPUT          ((CSA_flags)0x20000)
#define CSA_EXT_LAST_ELEMENT    ((CSA_flags)0x40000)
/* CALENDAR USER TYPE */
#define CSA_USER_TYPE_INDIVIDUAL        ((CSA_enum)0)
#define CSA_USER_TYPE_GROUP             ((CSA_enum)1)
#define CSA_USER_TYPE_RESOURCE          ((CSA_enum)2)
/* CLASSIFICATION
#define CSA_CLASS_PUBLIC                ((CSA_enum)0)
#define CSA_CLASS_PRIVATE               ((CSA_enum)1)
#define CSA_CLASS_CONFIDENTIAL          ((CSA_enum)2)
/* ACCESS RIGHTS FLAGS */
#define CSA_FREE_TIME_SEARCH                    ((CSA_flags)0x1)
#define CSA_VIEW_PUBLIC_ENTRIES                 ((CSA_flags)0x2)
#define CSA_VIEW_CONFIDENTIAL_ENTRIES           ((CSA_flags)0x4)
#define CSA_VIEW_PRIVATE_ENTRIES                ((CSA_flags)0x8)
#define CSA_INSERT_PUBLIC_ENTRIES               ((CSA_flags)0x10)
#define CSA_INSERT_CONFIDENTIAL_ENTRIES         ((CSA_flags)0x20)
#define CSA_INSERT_PRIVATE_ENTRIES              ((CSA_flags)0x40)
#define CSA_CHANGE_PUBLIC_ENTRIES               ((CSA_flags)0x80)
#define CSA_CHANGE_CONFIDENTIAL_ENTRIES         ((CSA_flags)0x100)
#define CSA_CHANGE_PRIVATE_ENTRIES              ((CSA_flags)0x200)
#define CSA_VIEW_CALENDAR_ATTRIBUTES            ((CSA_flags)0x400)
#define CSA_INSERT_CALENDAR_ATTRIBUTES          ((CSA_flags)0x800)
#define CSA_CHANGE_CALENDAR_ATTRIBUTES          ((CSA_flags)0x1000)
#define CSA_ORGANIZER_RIGHTS                    ((CSA_flags)0x2000)
#define CSA_SPONSOR_RIGHTS                      ((CSA_flags)0x4000)
#define CSA_OWNER_RIGHTS                        ((CSA_flags)0x8000)
/* ATTENDEE PRIORITIES */
#define CSA_FOR_YOUR_INFORMATION        ((CSA_enum)0)
#define CSA_ATTENDANCE_REQUESTED        ((CSA_enum)1)
#define CSA_ATTENDANCE_REQUIRED         ((CSA_enum)2)
#define CSA_IMMEDIATE_RESPONSE          ((CSA_enum)3)
/* ATTENDEE STATUS */
#define CSA_STATUS_ACCEPTED             ((CSA_enum)0)
#define CSA_STATUS_NEEDS_ACTION         ((CSA_enum)1)
#define CSA_STATUS_SENT                 ((CSA_enum)2)
#define CSA_STATUS_TENTATIVE            ((CSA_enum)3)
#define CSA_STATUS_CONFIRMED            ((CSA_enum)4)
#define CSA_STATUS_REJECTED             ((CSA_enum)5)
#define CSA_STATUS_COMPLETED            ((CSA_enum)6)
#define CSA_STATUS_DELEGATED            ((CSA_enum)7)
/* ATTRIBUTE VALUE TYPE */
#define CSA_VALUE_BOOLEAN                       ((CSA_enum) 0)
#define CSA_VALUE_ENUMERATED                    ((CSA_enum) 1)
#define CSA_VALUE_FLAGS                         ((CSA_enum) 2)
#define CSA_VALUE_SINT32                        ((CSA_enum) 3)
#define CSA_VALUE_UINT32                        ((CSA_enum) 4)
#define CSA_VALUE_STRING                        ((CSA_enum) 5)
#define CSA_VALUE_CALENDAR_USER                 ((CSA_enum) 6)
#define CSA_VALUE_DATE_TIME                     ((CSA_enum) 7)
#define CSA_VALUE_DATE_TIME_RANGE               ((CSA_enum) 8)
#define CSA_VALUE_TIME_DURATION                 ((CSA_enum) 9)
#define CSA_VALUE_ACCESS_LIST                   ((CSA_enum) 10)
#define CSA_VALUE_ATTENDEE_LIST                 ((CSA_enum) 11)
#define CSA_VALUE_DATE_TIME_LIST                ((CSA_enum) 12)
#define CSA_VALUE_REMINDER                      ((CSA_enum) 13)
#define CSA_VALUE_OPAQUE_DATA                   ((CSA_enum) 14)
/* CROSS FUNCTION FLAGS */
/* SCOPE */
#define CSA_SCOPE_ALL                           ((CSA_enum)0)
#define CSA_SCOPE_ONE                           ((CSA_enum)1)
#define CSA_SCOPE_FORWARD                       ((CSA_enum)2)
/* OPERATORS */
#define CSA_MATCH_ANY                           ((CSA_enum)0)
#define CSA_MATCH_EQUAL_TO                      ((CSA_enum)1)
#define CSA_MATCH_NOT_EQUAL_TO                  ((CSA_enum)2)
#define CSA_MATCH_GREATER_THAN                  ((CSA_enum)3)
#define CSA_MATCH_LESS_THAN                     ((CSA_enum)4)
#define CSA_MATCH_GREATER_THAN_OR_EQUAL_TO      ((CSA_enum)5)
#define CSA_MATCH_LESS_THAN_OR_EQUAL_TO         ((CSA_enum)6)
#define CSA_MATCH_CONTAIN                       ((CSA_enum)7)
#define CSA_LOOKUP_RESOLVE_PREFIX_SEARCH        ((CSA_flags)0x1)
#define CSA_LOOKUP_RESOLVE_IDENTITY             ((CSA_flags)0x2)
#define CSA_CONFIG_CHARACTER_SET                ((CSA_enum)0)
#define CSA_CONFIG_LINE_TERM                    ((CSA_enum)1)
#define CSA_CONFIG_DEFAULT_SERVICE              ((CSA_enum)2)
#define CSA_CONFIG_DEFAULT_USER                 ((CSA_enum)3)
#define CSA_CONFIG_REQ_PASSWORD                 ((CSA_enum)4)
#define CSA_CONFIG_REQ_SERVICE                  ((CSA_enum)5)
#define CSA_CONFIG_REQ_USER                     ((CSA_enum)6)
#define CSA_CONFIG_UI_AVALI                     ((CSA_enum)7)
#define CSA_CONFIG_VER_IMPLEM                   ((CSA_enum)8)
#define CSA_CONFIG_VER_SPEC                     ((CSA_enum)9)
/* CHARACTER SET IDENTIFIERS */
#define CSA_CHARSET_437         "-//XAPIA//CHARSET IBM 437//EN"
#define CSA_CHARSET_850         "-//XAPIA//CHARSET IBM 850//EN"
#define CSA_CHARSET_1252        "-//XAPIA//CHARSET Microsoft 1252//EN"
#define CSA_CHARSET_ISTRING     "-//XAPIA//CHARSET Apple ISTRING//EN"
#define CSA_CHARSET_UNICODE     "-//XAPIA//CHARSET UNICODE//EN"
#define CSA_CHARSET_T61         "-//XAPIA//CHARSET TSS T61//EN"
#define CSA_CHARSET_IA5         "-//XAPIA//CHARSET TSS IA5//EN"
#define CSA_CHARSET_ISO_10646   "-//XAPIA//CHARSET ISO 10646//EN"
#define CSA_CHARSET_ISO_646     "-//XAPIA//CHARSET ISO 646//EN"
#define CSA_CHARSET_ISO_8859_1  "-//XAPIA//CHARSET ISO 8859-1//EN"
#define CSA_CB_CALENDAR_LOGON                   ((CSA_flags)0x0
#define CSA_CB_CALENDAR_DELETED                 ((CSA_flags)0x1)
#define CSA_CB_CALENDAR_ATTRIBUTE_UPDATED       ((CSA_flags)0x2)
#define CSA_CB_ENTRY_ADDED                      ((CSA_flags)0x4)
#define CSA_CB_ENTRY_DELETED                    ((CSA_flags)0x8)
#define CSA_CB_ENTRY_UPDATED                    ((CSA_flags)0x10)
/* Entry Types */
#define CSA_TYPE_EVENT          ((CSA_enum)0)
#define CSA_TYPE_TODO           ((CSA_enum)1)
#define CSA_TYPE_MEMO           ((CSA_enum)2)
/* RETURN ERROR FLAGS */
#define CSA_ERROR_RSV_MASK      ((CSA_return_code)0x0000FFFF)
#define CSA_ERROR_IMPL_MASK     ((CSA_return_code)0xFFFF0000)
/* RETURN CODES */
#define CSA_SUCCESS                             ((CSA_return_code)0)
#define CSA_E_AMBIGUOUS_USER                    ((CSA_return_code)1)
#define CSA_E_CALENDAR_EXISTS                   ((CSA_return_code)2)
#define CSA_E_CALENDAR_NOT_EXIST                ((CSA_return_code)3)
#define CSA_E_CALLBACK_NOT_REGISTERED           ((CSA_return_code)4)
#define CSA_E_DISK_FULL                         ((CSA_return_code)5)
#define CSA_E_FAILURE                           ((CSA_return_code)6)
#define CSA_E_FILE_EXIST                        ((CSA_return_code)7)
#define CSA_E_FILE_NOT_EXIST                    ((CSA_return_code)8)
#define CSA_E_INSUFFICIENT_MEMORY               ((CSA_return_code)9)
#define CSA_E_INVALID_ATTRIBUTE                 ((CSA_return_code)10)
#define CSA_E_INVALID_ATTRIBUTE_VALUE           ((CSA_return_code)11)
#define CSA_E_INVALID_CALENDAR_SERVICE          ((CSA_return_code)12)
#define CSA_E_INVALID_CONFIGURATION             ((CSA_return_code)13)
#define CSA_E_INVALID_DATA_EXT                  ((CSA_return_code)14)
#define CSA_E_INVALID_DATE_TIME                 ((CSA_return_code)15)
#define CSA_E_INVALID_ENTRY_HANDLE              ((CSA_return_code)16)
#define CSA_E_INVALID_ENUM                      ((CSA_return_code)17)
#define CSA_E_INVALID_FILE_NAME                 ((CSA_return_code)18)
#define CSA_E_INVALID_FLAG                      ((CSA_return_code)19)
#define CSA_E_INVALID_FUNCTION_EXT              ((CSA_return_code)20)
#define CSA_E_INVALID_MEMORY                    ((CSA_return_code)21)
#define CSA_E_INVALID_PARAMETER                 ((CSA_return_code)22)
#define CSA_E_INVALID_PASSWORD                  ((CSA_return_code)23)
#define CSA_E_INVALID_RETURN_CODE               ((CSA_return_code)24)
#define CSA_E_INVALID_SESSION_HANDLE            ((CSA_return_code)25)
#define CSA_E_INVALID_USER                      ((CSA_return_code)26)
#define CSA_E_LOCALE_MISMATCH                   ((CSA_return_code)27)
#define CSA_E_LOGON_FAILURE                     ((CSA_return_code)28)
#define CSA_E_NO_AUTHORITY                      ((CSA_return_code)29)
#define CSA_E_NOT_SUPPORTED                     ((CSA_return_code)30)
#define CSA_E_PASSWORD_REQUIRED                 ((CSA_return_code)31)
#define CSA_E_READONLY                          ((CSA_return_code)32)
#define CSA_E_SERVICE_UNAVAILABLE               ((CSA_return_code)33)
#define CSA_E_TEXT_TOO_LARGE                    ((CSA_return_code)34)
#define CSA_E_TOO_MANY_USERS                    ((CSA_return_code)35)
#define CSA_E_UNABLE_TO_OPEN_FILE               ((CSA_return_code)36)
#define CSA_E_UNSUPPORTED_ATTRIBUTE             ((CSA_return_code)37)
#define CSA_E_UNSUPPORTED_CHARACTER_SET         ((CSA_return_code)38)
#define CSA_E_UNSUPPORTED_DATA_EXT              ((CSA_return_code)39)
#define CSA_E_UNSUPPORTED_FLAG                  ((CSA_return_code)40)
#define CSA_E_UNSUPPORTED_FUNCTION_EXT          ((CSA_return_code)41)
#define CSA_E_UNSUPPORTED_PARAMETER             ((CSA_return_code)42)
#define CSA_E_UNSUPPORTED_VERSION               ((CSA_return_code)43)
#define CSA_E_USER_NOT_FOUND                    ((CSA_return_code)44)
/* CALENDAR ATTRIBUTES */
/* CALENDAR ATTRIBUTE NAMES */
#define CSA_CAL_ATTR_ACCESS_LIST                              &bsol;
        "-//XAPIA/CSA/CALATTR//NONSGML Access List//EN"
#define CSA_CAL_ATTR_CALENDAR_NAME                            &bsol;
        "-//XAPIA/CSA/CALATTR//NONSGML Calendar Name//EN"
#define CSA_CAL_ATTR_CALENDAR_OWNER                           &bsol;
        "-//XAPIA/CSA/CALATTR//NONSGML Calendar Owner//EN"
#define CSA_CAL_ATTR_CALENDAR_SIZE                            &bsol;
        "-//XAPIA/CSA/CALATTR//NONSGML Calendar Size//EN"
#define CSA_CAL_ATTR_CODE_PAGE                                &bsol;
        "-//XAPIA/CSA/CALATTR//NONSGML Character Set//EN"
#define CSA_CAL_ATTR_COUNTRY                                  &bsol;
        "-//XAPIA/CSA/CALATTR//NONSGML Country//EN"
#define CSA_CAL_ATTR_DATE_CREATED                             &bsol;
        "-//XAPIA/CSA/CALATTR//NONSGML Date Created//EN"
#define CSA_CAL_ATTR_LANGUAGE                                 &bsol;
        "-//XAPIA/CSA/CALATTR//NONSGML Language//EN"
#define CSA_CAL_ATTR_NUMBER_ENTRIES                           &bsol;
        "-//XAPIA/CSA/CALATTR//NONSGML Number Entries//EN"
#define CSA_CAL_ATTR_PRODUCT_IDENTIFIER                       &bsol;
        "-//XAPIA/CSA/CALATTR//NONSGML Product Identifier//EN"
#define CSA_CAL_ATTR_TIME_ZONE                                &bsol;
        "-//XAPIA/CSA/CALATTR//NONSGML Time Zone//EN"
#define CSA_CAL_ATTR_VERSION                                  &bsol;
        "-//XAPIA/CSA/CALATTR//NONSGML Version//EN"
#define CSA_CAL_ATTR_WORK_SCHEDULE                            &bsol;
        "-//XAPIA/CSA/CALATTR//NONSGML Work Schedule//EN"
/* ENTRY ATTRIBUTES */
/* ENTRY ATTRIBUTES NAMES */
#define CSA_ENTRY_ATTR_ATTENDEE_LIST                          &bsol;
        "-//XAPIA/CSA/ENTRYATTR//NONSGML Attendee List//EN"
#define CSA_ENTRY_ATTR_AUDIO_REMINDER                         &bsol;
        "-//XAPIA/CSA/ENTRYATTR//NONSGML Audio Reminder//EN"
#define CSA_ENTRY_ATTR_CLASSIFICATION                         &bsol;
        "-//XAPIA/CSA/ENTRYATTR//NONSGML Classification//EN"
#define CSA_ENTRY_ATTR_DATE_COMPLETED                         &bsol;
        "-//XAPIA/CSA/ENTRYATTR//NONSGML Date Complated//EN"
#define CSA_ENTRY_ATTR_DATE_CREATED                           &bsol;
        "-//XAPIA/CSA/ENTRYATTR//NONSGML Date Created//EN"
#define CSA_ENTRY_ATTR_DESCRIPTION                            &bsol;
        "-//XAPIA/CSA/ENTRYATTR//NONSGML Description//EN"
#define CSA_ENTRY_ATTR_DUE_DATE                               &bsol;
        "-//XAPIA/CSA/ENTRYATTR//NONSGML Due Date//EN"
#define CSA_ENTRY_ATTR_END_DATE                               &bsol;
        "-//XAPIA/CSA/ENTRYATTR//NONSGML End Date//EN"
#define CSA_ENTRY_ATTR_EXCEPTION_DATES                        &bsol;
        "-//XAPIA/CSA/ENTRYATTR//NONSGML Exception Dates//EN"
#define CSA_ENTRY_ATTR_EXCEPTION_RULE                         &bsol;
        "-//XAPIA/CSA/ENTRYATTR//NONSGML Exception Rule//EN"
#define CSA_ENTRY_ATTR_FLASHING_REMINDER                      &bsol;
        "-//XAPIA/CSA/ENTRYATTR//NONSGML Flashing Reminder//EN"
#define CSA_ENTRY_ATTR_LAST_UPDATE                            &bsol;
        "-//XAPIA/CSA/ENTRYATTR//NONSGML Last Update//EN"
#define CSA_ENTRY_ATTR_MAIL_REMINDER                          &bsol;
        "-//XAPIA/CSA/ENTRYATTR//NONSGML Mail Reminder//EN"
#define CSA_ENTRY_ATTR_NUMBER_RECURRENCES                     &bsol;
        "-//XAPIA/CSA/ENTRYATTR//NONSGML Number Recurrences//EN"
#define CSA_ENTRY_ATTR_ORGANIZER                              &bsol;
        "-//XAPIA/CSA/ENTRYATTR//NONSGML Organizer//EN"
#define CSA_ENTRY_ATTR_POPUP_REMINDER                         &bsol;
        "-//XAPIA/CSA/ENTRYATTR//NONSGML Popup Reminder//EN"
#define CSA_ENTRY_ATTR_PRIORITY                               &bsol;
        "-//XAPIA/CSA/ENTRYATTR//NONSGML Priority//EN"
#define CSA_ENTRY_ATTR_RECURRENCE_RULE                        &bsol;
        "-//XAPIA/CSA/ENTRYATTR//NONSGML Recurrence Rule//EN"
#define CSA_ENTRY_ATTR_RECURREING_DATES                       &bsol;
        "-//XAPIA/CSA/ENTRYATTR//NONSGML Recurring Dates//EN"
#define CSA_ENTRY_ATTR_REFERENCE_IDENTIFIER                   &bsol;
        "-//XAPIA/CSA/ENTRYATTR//NONSGML Reference_identifier//EN"
#define CSA_ENTRY_ATTR_SEQUENCE_NUMBER                        &bsol;
        "-//XAPIA/CSA/ENTRYATTR//NONSGML Sequence Number//EN"
#define CSA_ENTRY_ATTR_SPONSOR                                &bsol;
        "-//XAPIA/CSA/ENTRYATTR//NONSGML Sponsor//EN"
#define CSA_ENTRY_ATTR_START_DATE                             &bsol;
        "-//XAPIA/CSA/ENTRYATTR//NONSGML Start Date//EN"
#define CSA_ENTRY_ATTR_STATUS                                 &bsol;
        "-//XAPIA/CSA/ENTRYATTR//NONSGML Status//EN"
#define CSA_ENTRY_ATTR_SUBTYPE                                &bsol;
        "-//XAPIA/CSA/ENTRYATTR//NONSGML Subtype//EN"
#define CSA_ENTRY_ATTR_SUMMARY                                &bsol;
        "-//XAPIA/CSA/ENTRYATTR//NONSGML Summary//EN"
#define CSA_ENTRY_ATTR_TIME_TRANSPARENCY                      &bsol;
        "-//XAPIA/CSA/ENTRYATTR//NONSGML Transparency//EN"
#define CSA_ENTRY_ATTR_TYPE                                   &bsol;
        "-//XAPIA/CSA/ENTRYATTR//NONSGML Type//EN"
/* COMMON EXTENSIONS DECLARATIONS */
/* EXTENSION SET ID */
/* Common Extension Set */
#define CSA_XS_COM                      ((CSA_uint32) 0)
/* Bilateral Extension Set */
#define CSA_XS_BLT                      ((CSA_uint32) 256)
/* FUNCTION EXTENSIONS */
/* Query for extension support in implementation */
#define CSA_X_COM_SUPPORT_EXT           ((CSA_uint32) 1)
#define CSA_X_COM_SUPPORTED             ((CSA_flags) 1)
#define CSA_X_COM_NOT_SUPPORTED         ((CSA_flags) 2)
#define CSA_X_COM_DATA_EXT_SUPPORTED    ((CSA_flags) 4)
#define CSA_X_COM_FUNC_EXT_SUPPORTED    ((CSA_flags) 8)
#define CSA_X_COM_SUP_EXCLUDE           ((CSA_flags) 16)
#define CSA_X_UI_ID_EXT                 ((CSA_uint32)2)
/* EXTENSION FLAGS */
#define CSA_X_LOGON_UI_ALLOWED          ((CSA_flags)0x1)
#define CSA_X_ERROR_UI_ALLOWED          ((CSA_flags)0x2)
#define CSA_X_LOOKUP_RESOLVE_UI         ((CSA_flags)0x4)
#define CSA_X_LOOKUP_DETAILS_UI         ((CSA_flags)0x8)
#define CSA_X_LOOKUP_ADDRESSING_UI      ((CSA_flags)0x10)
#define CSA_X_ADD_DEFINE_ENTRY_UI       ((CSA_flags)0x20)
/* EXTENSION RETURN CODES */
#define CSA_X_E_INVALID_UI_ID           ((CSA_return_code)1025)
#define CSA_X_E_LOGON_FAILURE           ((CSA_return_code)1026)
#define CSA_X_E_USER_CANCEL             ((CSA_return_code)1027)
#define CSA_X_XT_APP_CONTEXT_EXT        ((CSA_uint32)3)

The header declares the following structures:typedef struct CSA_TAG_EXTENSION {
        CSA_uint32item_code;
        CSA_uint32item_data;
        CSA_bufferitem_reference;
        CSA_flagsextension_flags;
} CSA_extension;
typedef struct CSA_TAG_CALENDAR_USER {
        CSA_stringuser_name;
        CSA_enumuser_type;
        CSA_stringcalendar_address;
        CSA_extension *calendar_user_extensions;
} CSA_calendar_user;
typedef struct CSA_TAG_ACCESS_RIGHTS {
        CSA_calendar_user *user;
        CSA_flagsrights;
        struct CSA_TAG_ACCESS_RIGHTS *`next`;
} CSA_access_rights, *CSA_access_list;
typedef struct CSA_TAG_ATTENDEE {
        CSA_calendar_userattendee;
        CSA_enumpriority;
        CSA_enum`status`;
        CSA_booleanrsvp_requested;
        struct CSA_TAG_ATTENDEE *`next`;
} CSA_attendee, *CSA_attendee_list;
typedef struct CSA_TAG_OPAQUE_DATA {
        CSA_uint32`size`;
        CSA_uint8 *`data`;
} CSA_opaque_data;
typedef void (*CSA_callback)(
        CSA_session_handlesession,
        CSA_flags`reason`,
        CSA_buffer`call_data`,
        CSA_buffer`client_data`,
        CSA_extension *callback_extensions);
typedef struct CSA_TAG_LOGON_CB_DATA {
        CSA_calendar_user *user;
} CSA_logon_callback_data;
typedef struct CSA_TAG_CALENDAR_DELETED_CB_DATA {
        CSA_calendar_user *user;
} CSA_calendar_deleted_callback_data;
typedef struct CSA_TAG_CALENDAR_ATTR_UPDATE_CB_DATA {
        CSA_calendar_user *user;
        CSA_uint32number_attributes;
        CSA_attribute_reference *attribute_names;
} CSA_calendar_attr_update_callback_data;
typedef struct CSA_TAG_ADD_ENTRY_CB_DATA {
        CSA_calendar_user *user;
        CSA_opaque_dataadded_entry_id;
} CSA_add_entry_callback_data;
typedef struct CSA_TAG_DELETE_ENTRY_CB_DATA {
        CSA_calendar_user *user;
        CSA_opaque_datadeleted_entry_id;
        CSA_enumscope;
        CSA_date_timedate_and_time;
} CSA_delete_entry_callback_data;
typedef struct CSA_TAG_UPDATE_ENTRY_CB_DATA {
        CSA_calendar_user *user;
        CSA_opaque_dataold_entry_id;
        CSA_opaque_datanew_entry_id;
        CSA_enumscope;
        CSA_date_timedate_and_time;
} CSA_update_entry_callback_data;
typedef struct CSA_TAG_DATE_TIME_ITEM {
        CSA_date_timedate_time;
        struct CSA_TAG_DATE_TIME_ITEM *`next`;
} CSA_date_time_entry, *CSA_date_time_list;
typedef struct CSA_TAG_FREE_TIME {
        CSA_uint32number_free_time_data;
        CSA_date_time_range *free_time_data;
} CSA_free_time;
typedef struct CSA_TAG_REMINDER {
        CSA_time_durationlead_time;
        CSA_time_durationsnooze_time;
        CSA_uint32repeat_count;
        CSA_opaque_datareminder_data;
} CSA_reminder;
typedef struct CSA_TAG_REMINDER_REFERENCE {
        CSA_entry_handle`entry`;
        CSA_date_timerun_time;
        CSA_time_durationsnooze_time;
        CSA_uint32repeat_count;
        CSA_attribute_referenceattribute_name;
} CSA_reminder_reference;
typedef struct CSA_TAG_ATTRIBUTE_ITEM {
        CSA_enum`type`;
        union {
                CSA_booleanboolean_value;
                CSA_enumenumerated_value;
                CSA_flagsflags_value;
                CSA_sint32sint32_value;
                CSA_uint32uint32_value;
                CSA_calendar_usercalendar_user_value;
                CSA_date_timedate_time_value;
                CSA_date_time_rangedate_time_range_value;
                CSA_time_durationtime_duration_value;
                CSA_stringstring_value;
                CSA_attendee_listattendee_list_value;
                CSA_date_time_listdate_time_list_value;
                CSA_access_listaccess_list_value;
                CSA_reminder *reminder_value;
                CSA_opaque_data *opaque_data_value;
        } item;
} CSA_attribute_value;
typedef struct CSA_TAG_ATTRIBUTE {
        CSA_string`name`;
        CSA_attribute_value *`value`;
        CSA_extension *attribute_extensions;
} CSA_attribute;
typedef struct CSA_TAG_WORK_SCHEDULE {
        CSA_date_timeschedule_begin_time;
        CSA_booleancyclic_definition_flag;
        CSA_date_timecycle_end_time;
        CSA_date_time_list *work_cycle;
} CSA_work_schedule;
typedef struct CSA_TAG_XCOM {
        CSA_uint32item_code;
        CSA_flags`flags`;
} CSA_X_COM_support;

The header declares the following as functions:CSA_return_code csa_add_calendar(CSA_session_handlesession,
        CSA_calendar_user *user,
        CSA_uint32number_attributes,
        CSA_attribute *calendar_attributes,
        CSA_extension *add_calendar_extensions);CSA_return_code csa_add_entry(CSA_session_handlesession,
        CSA_uint32number_attributes,
        CSA_attribute *entry_attributes,
        CSA_entry_handle *`entry`,
        CSA_extension *add_entry_extensions);CSA_return_code csa_call_callbacks(CSA_session_handlesession,
        CSA_flags`reason`,
        CSA_extension *call_callbacks_extensions);CSA_return_code csa_delete_calendar(CSA_session_handlesession,
        CSA_extension *delete_calendar_extensions);CSA_return_code csa_delete_entry(CSA_session_handlesession,
        CSA_entry_handle`entry`,
        CSA_enumdelete_scope,
        CSA_extension *delete_entry_extensions);CSA_return_code csa_free(CSA_buffermemory);CSA_return_code csa_free_time_search(CSA_session_handlesession,
        CSA_date_time_rangedate_time_range,
        CSA_time_durationtime_duration,
        CSA_uint32number_users,
        CSA_calendar_user *calendar_users,
        CSA_free_time **free_time,
        CSA_extension *free_time_search_extensions);CSA_return_code csa_list_calendar_attributes(CSA_session_handlesession,
        CSA_uint32 *number_names,
        CSA_attribute_reference **calendar_attributes_names,
        CSA_extension *list_calendar_attributes_extensions);CSA_return_code csa_list_calendars(CSA_service_referencecalendar_service,
        CSA_uint32 *number_names,
        CSA_calendar_user **calendar_names,
        CSA_extension *list_calendars_extensions);CSA_return_code csa_list_entries(CSA_session_handlesession,
        CSA_uint32number_attributes,
        CSA_attribute *entry_attributesCSA_enum *list_operators,
        CSA_uint32 *number_entries,
        CSA_entry_handle **entries,
        CSA_extension *list_entries_extensions);CSA_return_code csa_list_entry_attributes(CSA_session_handlesession,
        CSA_entry_handle`entry`,
        CSA_uint32 *number_names,
        CSA_attribute_reference **entry_attributes_namesCSA_extension *list_entry_attributes_extensions);CSA_return_code csa_list_entry_sequence(CSA_session_handlesession,
        CSA_entry_handle`entry`,
        CSA_date_time_rangetime_range,
        CSA_uint32 *number_entries,
        CSA_entry_handle **entry_list,
        CSA_extension *list_entry_sequence_extensions);CSA_return_code csa_logoff(CSA_session_handlesession,
        CSA_extension *logoff_extensions);CSA_return_code csa_logon(CSA_service_referencecalendar_service,
        CSA_calendar_user *user,
        CSA_stringpassword,
        CSA_stringcharacter_set,
        CSA_stringrequired_csa_version,
        CSA_session_handle *session,
        CSA_extension *logon_extensions);CSA_return_code csa_look_up(CSA_session_handlesession,
        CSA_calendar_user *users,
        CSA_flagslook_up_flags,
        CSA_uint32 *number_users,
        CSA_calendar_user **user_list,
        CSA_extension *look_up_extensions);CSA_return_code csa_query_configuration(CSA_session_handlesession,
        CSA_enum`item`,
        CSA_buffer *reference,
        CSA_extension *query_configuration_extensions);CSA_return_code csa_read_calendar_attributes(CSA_session_handlesession,
        CSA_uint32number_names,
        CSA_attribute_reference *attribute_names,
        CSA_uint32 *number_attributes,
        CSA_attribute **calendar_attributesCSA_extension *read_calendar_attributes_extensions);CSA_return_code csa_read_entry_attributes(CSA_session_handlesession,
        CSA_entry_handle`entry`,
        CSA_uint32number_names,
        CSA_attribute_reference *attribute_names,
        CSA_uint32 *number_attributes,
        CSA_attribute **entry_attributes,
        CSA_extension *read_entry_attributes_extensions);CSA_return_code csa_read_next_reminder(CSA_session_handlesession,
        CSA_uint32number_names,
        CSA_attribute_reference *reminder_names,
        CSA_date_timegiven_time,
        CSA_uint32 *number_remindersCSA_reminder_reference **reminder_references,
        CSA_extension *read_next_reminder_extensions);CSA_return_code csa_register_callback(CSA_session_handlesession,
        CSA_flags`reason`,
        CSA_callback`callback`,
        CSA_buffer`client_data`,
        CSA_extension *register_callback_extensions);CSA_return_code csa_restore(CSA_session_handlesession,
        CSA_stringarchive_name,
        CSA_uint32number_attributes,
        CSA_attribute *attributes,
        CSA_enum *operators,
        CSA_extension *restore_extensions);CSA_return_code csa_save(CSA_session_handlesession,
        CSA_stringarchive_name,
        CSA_uint32number_attributesCSA_attribute *attributes,
        CSA_enum *operators,
        CSA_booleandelete_entry,
        CSA_extension *save_extensions);CSA_return_code csa_unregister_callback(CSA_session_handlesession,
        CSA_flags`reason`,
        CSA_callback`callback`,
        CSA_buffer`client_data`,
        CSA_extension *unregister_callback_extensions);CSA_return_code csa_update_calendar_attributes(CSA_session_handlesession,
        CSA_uint32number_attributes,
        CSA_attribute *calendar_attributes,
        CSA_extension *update_calendar_attributes_extensions);CSA_return_code csa_update_entry_attributes(CSA_session_handlesession,
        CSA_entry_handle`entry`,
        CSA_enumupdate_scope,
        CSA_booleanupdate_propagation,
        CSA_uint32number_attributes,
        CSA_attribute *entry_attributes,
        CSA_entry_handle *new_entry,
        CSA_extension *update_entry_attributes_extensions);