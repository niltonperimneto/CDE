# dttypes
user cmddttypesGenerates a list of the DT Action and DataTypes definitions.dttypes-helpdttypes-type filenamedttypes-db database-wrec_name regexpfld_name regexpfld_value regexp-lrec_namerec_infofld_name regexpfld_value
## DESCRIPTION


Dttypes is a client that lists the Action and DataTypes definitions. This is
useful in understanding where DT is getting the information for its
databases and how it is using that information to construct the databases.
By default it prints out the entire set of databases.
## OPTIONS


The optional command_list is composed of one or more of the following:-helpprints out the usage message.-type filenamewhere filename is the name of a file to be typed.-db databasewhere database uses all the DataBases whose name matches the regular expression database. Currently: DATA_CRITERIA, DATA_ATTRIBUTES or ACTION.-w search_listwhere search_list consists of one or more of the following:rec_name reg_exp - finds all records whose name matches the regular expression reg_exp.fld_name reg_exp - finds all records whose field name matches the regular expression reg_exp.fld_value reg_exp - finds all records whose field values matches the regular expression reg_exp.-l display_listwhere display_list consist of one or more of the following:rec_name - displays the name of the records found.rec_info - displays the file the record was found in.fld_name [reg_exp] - displays the names of the attributes found in that record.If a reg_exp is specified then only those fields that match will be displayed.fld_value - displays the values of the attributes found in the record.
## SEE


&cdeman.DtDtsLoadDataTypes;,
&cdeman.DtDtsDataToDataType;,
&cdeman.DtDtsFileToDataType;,
&cdeman.DtDtsFileToAttributeList;,
&cdeman.DtDtsFileToAttributeValue;,
&cdeman.DtDtsBufferToDataType;,
&cdeman.DtDtsBufferToAttributeList;,
&cdeman.DtDtsBufferToAttributeValue;,
&cdeman.DtDtsDataTypeToAttributeList;,
&cdeman.DtDtsDataTypeToAttributeValue;,
&cdeman.DtDtsFreeDataType;,
&cdeman.DtDtsFreeAttributeList;,
&cdeman.DtDtsFreeAttributeValue;,
&cdeman.DtDtsRelease;,
&cdeman.DtDtsDataTypeNames;,
&cdeman.DtDtsFindAttribute;,
&cdeman.DtDtsFreeDataTypeNames;,
&cdeman.DtDtsSetDataType;,
&cdeman.DtDtsDataTypeIsAction;,
&cdeman.DtActionLabel;,
&cdeman.DtActionDescription;,
&cdeman.DtActionExists;,
&cdeman.DtActionInvoke;,
&cdeman.dtaction;