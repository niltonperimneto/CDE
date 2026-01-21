DtDndProtocolspecial fileDtDndProtocoldrag and drop matching and transfer protocolsDESCRIPTIONThe drag and drop protocols provide policy for matching and data transfer
between the drag initiator and the drop receiver of file names, selected text
spans and application-defined structured data formats.The drag and drop protocols use the standard X11 selection targets,
where available, with the addition of several new selection targets where
required.These protocols provide for the transfer of the following types of data:Selected TextFile NamesBuffersPROTOCOL OVERVIEWEach protocol consists of the following:Drag and Drop API ProtocolEach protocol described corresponds to a specificDtDndProtocolenumeration value.Export/Import TargetsThe Motif drag and drop API provides support for matching of the data
transfer protocol between the drag initiator and the various drop receivers.
This allows the user to determine readily which drop sites will accept the
dragged data.The drag initiator sets theXmNexportTargetsresource of theXmDragContextto the
list of target atoms that describe the data being dragged. The drop receiver
sets theXmNimportTargetsresource
of the`XmDropSite`to the list of target atoms that describe
the data that it will accept. The Motif drag and drop subsystem allows drops
when theXmNexportTargetsandXmNimportTargetshave at least one target in
common.Data Transfer ProtocolOnce the drag initiator has dropped on the drop receiver, the transfer
of data is begun. The transfer is accomplished using X selections and is controlled
by the drop receiver.The drop receiver starts all transfers by converting the selection into
the ICCCMTARGETStarget to get
the set of available selection targets. (See the &str-Zi; for a description
of converting targets.) It then chooses the appropriate selections from that
set and requests that the drag initiator convert each requested selection.
Each protocol has a set of selection targets that are used to transfer all
the necessary data. These target conversions are usually initiated by callingXmDropTransferStart.Move CompletionWhen the operation of the drop isXmDROP_MOVE, the drop receiver must complete the move using an appropriate
method. For most data transfers, this is accomplished by converting the selection
into the ICCCMDELETEtarget to
tell the drag initiator that it may delete the data. For most file name transfers,
this is accomplished via the file system.TEXT TRANSFER PROTOCOLThe text transfer protocol is used to exchange text selections.Drag and Drop APIThis is the protocol used when aDtDndProtocolofDtDND_TEXT_TRANSFERis specified.Export/Import TargetsThe export or import targets are any of the following; the target describing
the character encoding of the text selection,COMPOUND_TEXT,STRINGorTEXT.Data Transfer ProtocolThe transfer of text selections follows the protocols described in the
ICCCM manual. If the character encoding of the drag initiator and drop receiver
are the same, that target should be converted to get the text selection. If
the character encoding are different, the drop receiver should attempt to
convert the standard text targets in the following order:COMPOUND_TEXT,STRINGorTEXT.Move CompletionThe move is completed by converting the selection into the ICCCMDELETEtarget.FILE NAME TRANSFER PROTOCOLThe transfer protocol is used to exchange file names.Drag and Drop APIThis is the protocol used when aDtDndProtocolofDtDND_FILENAME_TRANSFERis specified.Export/Import TargetsThe export or import targets areFILE_NAMEand, optionally,_DT_NETFILEif capable of providing the file name in network canonical form using &cdeman.tt.file.netfile; and &cdeman.tt.netfile.file;.Data Transfer ProtocolIf the ICCCMHOST_NAMEtarget
is in the list of target atoms, it is converted. If the returned host name
is different than the host name for the drop receiver and the_DT_NETFILEtarget is in the list of target
atoms, it is converted. The drag initiator uses &cdeman.tt.file.netfile; to encode the file names and the drop receiver uses &cdeman.tt.netfile.file; to decode the file names.If the hosts are the same for both the drag initiator and the drop receiver
or either theHOST_NAMEor the_DT_NETFILEtargets are not in the list of target
atoms from the drag initiator, the drop receiver converts the ICCCMFILE_NAMEtarget. No encoding of the file names
occurs in this case.Move CompletionMoves of file names can be accomplished atomically using standard file
system operations. Drop receivers are encouraged to use the file system. The
drop receiver may alternatively choose to use the ICCCMDELETEtarget to complete theXmDROP_MOVEand the drag initiator must be ready to comply.BUFFER TRANSFER PROTOCOLThe transfer protocol is used to exchange memory buffers.Drag and Drop APIThis is the protocol used when aDtDndProtocolofDtDND_BUFFER_TRANSFERis specified.Export/Import TargetsThe export and import targets are_DT_BUFFER_DATA,_DT_BUFFER_LENGTHSand, optionally,_DT_BUFFER_NAMES.Data Transfer ProtocolThe_DT_BUFFER_DATAand_DT_BUFFER_LENGTHStargets are converted to
transfer the buffer data.The data of the buffers is encoded into the_DT_BUFFER_DATAtarget as an array of bytes. The lengths in bytes
of each buffer are encoded into_DT_BUFFER_LENGTHS. Each length is used to index into the_DT_BUFFER_DATAarray.If the_DT_BUFFER_NAMEStarget
is available, it is converted to transfer the names of the buffers.Move CompletionThe move is completed by converting the selection into the ICCCMDELETEtarget.SELECTION TARGETSThe following table describes the selection targets used in the drag
and drop data matching and transfer protocols.