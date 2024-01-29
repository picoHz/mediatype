//! Predefined names, @generated in tests/codegen.rs
//!
//! # Sources
//! - <https://en.wikipedia.org/wiki/Media_type>
//! - <https://www.iana.org/assignments/media-types/media-types.xhtml>
//! - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_type>
//! - <https://developer.mozilla.org/en-US/docs/Glossary/Quality_values>
//! - <https://datatracker.ietf.org/doc/html/rfc3676>
//! - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types>

/// Vendor subtypes starting with `vnd.`.
pub mod vnd {
    /// `vnd.1000minds.decision-model`
    pub const _1000MINDS_DECISION_MODEL: crate::Name = crate::Name::new_unchecked("vnd.1000minds.decision-model");
    /// `vnd.3gpp-prose`
    pub const _3GPP_PROSE: crate::Name = crate::Name::new_unchecked("vnd.3gpp-prose");
    /// `vnd.3gpp-prose-pc3ch`
    pub const _3GPP_PROSE_PC3CH: crate::Name = crate::Name::new_unchecked("vnd.3gpp-prose-pc3ch");
    /// `vnd.3gpp-v2x-local-service-information`
    pub const _3GPP_V2X_LOCAL_SERVICE_INFORMATION: crate::Name = crate::Name::new_unchecked("vnd.3gpp-v2x-local-service-information");
    /// `vnd.3gpp.5gnas`
    pub const _3GPP_5GNAS: crate::Name = crate::Name::new_unchecked("vnd.3gpp.5gnas");
    /// `vnd.3gpp.access-transfer-events`
    pub const _3GPP_ACCESS_TRANSFER_EVENTS: crate::Name = crate::Name::new_unchecked("vnd.3gpp.access-transfer-events");
    /// `vnd.3gpp.bsf`
    pub const _3GPP_BSF: crate::Name = crate::Name::new_unchecked("vnd.3gpp.bsf");
    /// `vnd.3gpp.GMOP`
    pub const _3GPP_GMOP: crate::Name = crate::Name::new_unchecked("vnd.3gpp.GMOP");
    /// `vnd.3gpp.gtpc`
    pub const _3GPP_GTPC: crate::Name = crate::Name::new_unchecked("vnd.3gpp.gtpc");
    /// `vnd.3gpp.interworking-data`
    pub const _3GPP_INTERWORKING_DATA: crate::Name = crate::Name::new_unchecked("vnd.3gpp.interworking-data");
    /// `vnd.3gpp.iufp`
    pub const _3GPP_IUFP: crate::Name = crate::Name::new_unchecked("vnd.3gpp.iufp");
    /// `vnd.3gpp.lpp`
    pub const _3GPP_LPP: crate::Name = crate::Name::new_unchecked("vnd.3gpp.lpp");
    /// `vnd.3gpp.mc-signalling-ear`
    pub const _3GPP_MC_SIGNALLING_EAR: crate::Name = crate::Name::new_unchecked("vnd.3gpp.mc-signalling-ear");
    /// `vnd.3gpp.mcdata-affiliation-command`
    pub const _3GPP_MCDATA_AFFILIATION_COMMAND: crate::Name = crate::Name::new_unchecked("vnd.3gpp.mcdata-affiliation-command");
    /// `vnd.3gpp.mcdata-info`
    pub const _3GPP_MCDATA_INFO: crate::Name = crate::Name::new_unchecked("vnd.3gpp.mcdata-info");
    /// `vnd.3gpp.mcdata-payload`
    pub const _3GPP_MCDATA_PAYLOAD: crate::Name = crate::Name::new_unchecked("vnd.3gpp.mcdata-payload");
    /// `vnd.3gpp.mcdata-service-config`
    pub const _3GPP_MCDATA_SERVICE_CONFIG: crate::Name = crate::Name::new_unchecked("vnd.3gpp.mcdata-service-config");
    /// `vnd.3gpp.mcdata-signalling`
    pub const _3GPP_MCDATA_SIGNALLING: crate::Name = crate::Name::new_unchecked("vnd.3gpp.mcdata-signalling");
    /// `vnd.3gpp.mcdata-ue-config`
    pub const _3GPP_MCDATA_UE_CONFIG: crate::Name = crate::Name::new_unchecked("vnd.3gpp.mcdata-ue-config");
    /// `vnd.3gpp.mcdata-user-profile`
    pub const _3GPP_MCDATA_USER_PROFILE: crate::Name = crate::Name::new_unchecked("vnd.3gpp.mcdata-user-profile");
    /// `vnd.3gpp.mcptt-affiliation-command`
    pub const _3GPP_MCPTT_AFFILIATION_COMMAND: crate::Name = crate::Name::new_unchecked("vnd.3gpp.mcptt-affiliation-command");
    /// `vnd.3gpp.mcptt-floor-request`
    pub const _3GPP_MCPTT_FLOOR_REQUEST: crate::Name = crate::Name::new_unchecked("vnd.3gpp.mcptt-floor-request");
    /// `vnd.3gpp.mcptt-info`
    pub const _3GPP_MCPTT_INFO: crate::Name = crate::Name::new_unchecked("vnd.3gpp.mcptt-info");
    /// `vnd.3gpp.mcptt-location-info`
    pub const _3GPP_MCPTT_LOCATION_INFO: crate::Name = crate::Name::new_unchecked("vnd.3gpp.mcptt-location-info");
    /// `vnd.3gpp.mcptt-mbms-usage-info`
    pub const _3GPP_MCPTT_MBMS_USAGE_INFO: crate::Name = crate::Name::new_unchecked("vnd.3gpp.mcptt-mbms-usage-info");
    /// `vnd.3gpp.mcptt-service-config`
    pub const _3GPP_MCPTT_SERVICE_CONFIG: crate::Name = crate::Name::new_unchecked("vnd.3gpp.mcptt-service-config");
    /// `vnd.3gpp.mcptt-signed`
    pub const _3GPP_MCPTT_SIGNED: crate::Name = crate::Name::new_unchecked("vnd.3gpp.mcptt-signed");
    /// `vnd.3gpp.mcptt-ue-config`
    pub const _3GPP_MCPTT_UE_CONFIG: crate::Name = crate::Name::new_unchecked("vnd.3gpp.mcptt-ue-config");
    /// `vnd.3gpp.mcptt-ue-init-config`
    pub const _3GPP_MCPTT_UE_INIT_CONFIG: crate::Name = crate::Name::new_unchecked("vnd.3gpp.mcptt-ue-init-config");
    /// `vnd.3gpp.mcptt-user-profile`
    pub const _3GPP_MCPTT_USER_PROFILE: crate::Name = crate::Name::new_unchecked("vnd.3gpp.mcptt-user-profile");
    /// `vnd.3gpp.mcvideo-affiliation-command`
    pub const _3GPP_MCVIDEO_AFFILIATION_COMMAND: crate::Name = crate::Name::new_unchecked("vnd.3gpp.mcvideo-affiliation-command");
    /// `vnd.3gpp.mcvideo-affiliation-info`
    pub const _3GPP_MCVIDEO_AFFILIATION_INFO: crate::Name = crate::Name::new_unchecked("vnd.3gpp.mcvideo-affiliation-info");
    /// `vnd.3gpp.mcvideo-info`
    pub const _3GPP_MCVIDEO_INFO: crate::Name = crate::Name::new_unchecked("vnd.3gpp.mcvideo-info");
    /// `vnd.3gpp.mcvideo-location-info`
    pub const _3GPP_MCVIDEO_LOCATION_INFO: crate::Name = crate::Name::new_unchecked("vnd.3gpp.mcvideo-location-info");
    /// `vnd.3gpp.mcvideo-mbms-usage-info`
    pub const _3GPP_MCVIDEO_MBMS_USAGE_INFO: crate::Name = crate::Name::new_unchecked("vnd.3gpp.mcvideo-mbms-usage-info");
    /// `vnd.3gpp.mcvideo-service-config`
    pub const _3GPP_MCVIDEO_SERVICE_CONFIG: crate::Name = crate::Name::new_unchecked("vnd.3gpp.mcvideo-service-config");
    /// `vnd.3gpp.mcvideo-transmission-request`
    pub const _3GPP_MCVIDEO_TRANSMISSION_REQUEST: crate::Name = crate::Name::new_unchecked("vnd.3gpp.mcvideo-transmission-request");
    /// `vnd.3gpp.mcvideo-ue-config`
    pub const _3GPP_MCVIDEO_UE_CONFIG: crate::Name = crate::Name::new_unchecked("vnd.3gpp.mcvideo-ue-config");
    /// `vnd.3gpp.mcvideo-user-profile`
    pub const _3GPP_MCVIDEO_USER_PROFILE: crate::Name = crate::Name::new_unchecked("vnd.3gpp.mcvideo-user-profile");
    /// `vnd.3gpp.mid-call`
    pub const _3GPP_MID_CALL: crate::Name = crate::Name::new_unchecked("vnd.3gpp.mid-call");
    /// `vnd.3gpp.ngap`
    pub const _3GPP_NGAP: crate::Name = crate::Name::new_unchecked("vnd.3gpp.ngap");
    /// `vnd.3gpp.pfcp`
    pub const _3GPP_PFCP: crate::Name = crate::Name::new_unchecked("vnd.3gpp.pfcp");
    /// `vnd.3gpp.pic-bw-large`
    pub const _3GPP_PIC_BW_LARGE: crate::Name = crate::Name::new_unchecked("vnd.3gpp.pic-bw-large");
    /// `vnd.3gpp.pic-bw-small`
    pub const _3GPP_PIC_BW_SMALL: crate::Name = crate::Name::new_unchecked("vnd.3gpp.pic-bw-small");
    /// `vnd.3gpp.pic-bw-var`
    pub const _3GPP_PIC_BW_VAR: crate::Name = crate::Name::new_unchecked("vnd.3gpp.pic-bw-var");
    /// `vnd.3gpp.s1ap`
    pub const _3GPP_S1AP: crate::Name = crate::Name::new_unchecked("vnd.3gpp.s1ap");
    /// `vnd.3gpp.sms`
    pub const _3GPP_SMS: crate::Name = crate::Name::new_unchecked("vnd.3gpp.sms");
    /// `vnd.3gpp.srvcc-ext`
    pub const _3GPP_SRVCC_EXT: crate::Name = crate::Name::new_unchecked("vnd.3gpp.srvcc-ext");
    /// `vnd.3gpp.SRVCC-info`
    pub const _3GPP_SRVCC_INFO: crate::Name = crate::Name::new_unchecked("vnd.3gpp.SRVCC-info");
    /// `vnd.3gpp.state-and-event-info`
    pub const _3GPP_STATE_AND_EVENT_INFO: crate::Name = crate::Name::new_unchecked("vnd.3gpp.state-and-event-info");
    /// `vnd.3gpp.ussd`
    pub const _3GPP_USSD: crate::Name = crate::Name::new_unchecked("vnd.3gpp.ussd");
    /// `vnd.3gpp2.bcmcsinfo`
    pub const _3GPP2_BCMCSINFO: crate::Name = crate::Name::new_unchecked("vnd.3gpp2.bcmcsinfo");
    /// `vnd.3gpp2.sms`
    pub const _3GPP2_SMS: crate::Name = crate::Name::new_unchecked("vnd.3gpp2.sms");
    /// `vnd.3gpp2.tcap`
    pub const _3GPP2_TCAP: crate::Name = crate::Name::new_unchecked("vnd.3gpp2.tcap");
    /// `vnd.3lightssoftware.imagescal`
    pub const _3LIGHTSSOFTWARE_IMAGESCAL: crate::Name = crate::Name::new_unchecked("vnd.3lightssoftware.imagescal");
    /// `vnd.3M.Post-it-Notes`
    pub const _3M_POST_IT_NOTES: crate::Name = crate::Name::new_unchecked("vnd.3M.Post-it-Notes");
    /// `vnd.4SB`
    pub const _4SB: crate::Name = crate::Name::new_unchecked("vnd.4SB");
    /// `vnd.a`
    pub const A: crate::Name = crate::Name::new_unchecked("vnd.a");
    /// `vnd.abc`
    pub const ABC: crate::Name = crate::Name::new_unchecked("vnd.abc");
    /// `vnd.accpac.simply.aso`
    pub const ACCPAC_SIMPLY_ASO: crate::Name = crate::Name::new_unchecked("vnd.accpac.simply.aso");
    /// `vnd.accpac.simply.imp`
    pub const ACCPAC_SIMPLY_IMP: crate::Name = crate::Name::new_unchecked("vnd.accpac.simply.imp");
    /// `vnd.acucobol`
    pub const ACUCOBOL: crate::Name = crate::Name::new_unchecked("vnd.acucobol");
    /// `vnd.acucorp`
    pub const ACUCORP: crate::Name = crate::Name::new_unchecked("vnd.acucorp");
    /// `vnd.adobe.flash.movie`
    pub const ADOBE_FLASH_MOVIE: crate::Name = crate::Name::new_unchecked("vnd.adobe.flash.movie");
    /// `vnd.adobe.formscentral.fcdt`
    pub const ADOBE_FORMSCENTRAL_FCDT: crate::Name = crate::Name::new_unchecked("vnd.adobe.formscentral.fcdt");
    /// `vnd.adobe.fxp`
    pub const ADOBE_FXP: crate::Name = crate::Name::new_unchecked("vnd.adobe.fxp");
    /// `vnd.adobe.partial-upload`
    pub const ADOBE_PARTIAL_UPLOAD: crate::Name = crate::Name::new_unchecked("vnd.adobe.partial-upload");
    /// `vnd.adobe.photoshop`
    pub const ADOBE_PHOTOSHOP: crate::Name = crate::Name::new_unchecked("vnd.adobe.photoshop");
    /// `vnd.adobe.xdp`
    pub const ADOBE_XDP: crate::Name = crate::Name::new_unchecked("vnd.adobe.xdp");
    /// `vnd.adobe.xfdf`
    pub const ADOBE_XFDF: crate::Name = crate::Name::new_unchecked("vnd.adobe.xfdf");
    /// `vnd.aether.imp`
    pub const AETHER_IMP: crate::Name = crate::Name::new_unchecked("vnd.aether.imp");
    /// `vnd.afpc.afplinedata`
    pub const AFPC_AFPLINEDATA: crate::Name = crate::Name::new_unchecked("vnd.afpc.afplinedata");
    /// `vnd.afpc.afplinedata-pagedef`
    pub const AFPC_AFPLINEDATA_PAGEDEF: crate::Name = crate::Name::new_unchecked("vnd.afpc.afplinedata-pagedef");
    /// `vnd.afpc.cmoca-cmresource`
    pub const AFPC_CMOCA_CMRESOURCE: crate::Name = crate::Name::new_unchecked("vnd.afpc.cmoca-cmresource");
    /// `vnd.afpc.foca-charset`
    pub const AFPC_FOCA_CHARSET: crate::Name = crate::Name::new_unchecked("vnd.afpc.foca-charset");
    /// `vnd.afpc.foca-codedfont`
    pub const AFPC_FOCA_CODEDFONT: crate::Name = crate::Name::new_unchecked("vnd.afpc.foca-codedfont");
    /// `vnd.afpc.foca-codepage`
    pub const AFPC_FOCA_CODEPAGE: crate::Name = crate::Name::new_unchecked("vnd.afpc.foca-codepage");
    /// `vnd.afpc.modca`
    pub const AFPC_MODCA: crate::Name = crate::Name::new_unchecked("vnd.afpc.modca");
    /// `vnd.afpc.modca-cmtable`
    pub const AFPC_MODCA_CMTABLE: crate::Name = crate::Name::new_unchecked("vnd.afpc.modca-cmtable");
    /// `vnd.afpc.modca-formdef`
    pub const AFPC_MODCA_FORMDEF: crate::Name = crate::Name::new_unchecked("vnd.afpc.modca-formdef");
    /// `vnd.afpc.modca-mediummap`
    pub const AFPC_MODCA_MEDIUMMAP: crate::Name = crate::Name::new_unchecked("vnd.afpc.modca-mediummap");
    /// `vnd.afpc.modca-objectcontainer`
    pub const AFPC_MODCA_OBJECTCONTAINER: crate::Name = crate::Name::new_unchecked("vnd.afpc.modca-objectcontainer");
    /// `vnd.afpc.modca-overlay`
    pub const AFPC_MODCA_OVERLAY: crate::Name = crate::Name::new_unchecked("vnd.afpc.modca-overlay");
    /// `vnd.afpc.modca-pagesegment`
    pub const AFPC_MODCA_PAGESEGMENT: crate::Name = crate::Name::new_unchecked("vnd.afpc.modca-pagesegment");
    /// `vnd.age`
    pub const AGE: crate::Name = crate::Name::new_unchecked("vnd.age");
    /// `vnd.ah-barcode`
    pub const AH_BARCODE: crate::Name = crate::Name::new_unchecked("vnd.ah-barcode");
    /// `vnd.ahead.space`
    pub const AHEAD_SPACE: crate::Name = crate::Name::new_unchecked("vnd.ahead.space");
    /// `vnd.airzip.accelerator.azv`
    pub const AIRZIP_ACCELERATOR_AZV: crate::Name = crate::Name::new_unchecked("vnd.airzip.accelerator.azv");
    /// `vnd.airzip.filesecure.azf`
    pub const AIRZIP_FILESECURE_AZF: crate::Name = crate::Name::new_unchecked("vnd.airzip.filesecure.azf");
    /// `vnd.airzip.filesecure.azs`
    pub const AIRZIP_FILESECURE_AZS: crate::Name = crate::Name::new_unchecked("vnd.airzip.filesecure.azs");
    /// `vnd.amadeus`
    pub const AMADEUS: crate::Name = crate::Name::new_unchecked("vnd.amadeus");
    /// `vnd.amazon.ebook`
    pub const AMAZON_EBOOK: crate::Name = crate::Name::new_unchecked("vnd.amazon.ebook");
    /// `vnd.amazon.mobi8-ebook`
    pub const AMAZON_MOBI8_EBOOK: crate::Name = crate::Name::new_unchecked("vnd.amazon.mobi8-ebook");
    /// `vnd.americandynamics.acc`
    pub const AMERICANDYNAMICS_ACC: crate::Name = crate::Name::new_unchecked("vnd.americandynamics.acc");
    /// `vnd.amiga.ami`
    pub const AMIGA_AMI: crate::Name = crate::Name::new_unchecked("vnd.amiga.ami");
    /// `vnd.amundsen.maze`
    pub const AMUNDSEN_MAZE: crate::Name = crate::Name::new_unchecked("vnd.amundsen.maze");
    /// `vnd.android.ota`
    pub const ANDROID_OTA: crate::Name = crate::Name::new_unchecked("vnd.android.ota");
    /// `vnd.anki`
    pub const ANKI: crate::Name = crate::Name::new_unchecked("vnd.anki");
    /// `vnd.anser-web-certificate-issue-initiation`
    pub const ANSER_WEB_CERTIFICATE_ISSUE_INITIATION: crate::Name = crate::Name::new_unchecked("vnd.anser-web-certificate-issue-initiation");
    /// `vnd.antix.game-component`
    pub const ANTIX_GAME_COMPONENT: crate::Name = crate::Name::new_unchecked("vnd.antix.game-component");
    /// `vnd.apache.arrow.file`
    pub const APACHE_ARROW_FILE: crate::Name = crate::Name::new_unchecked("vnd.apache.arrow.file");
    /// `vnd.apache.arrow.stream`
    pub const APACHE_ARROW_STREAM: crate::Name = crate::Name::new_unchecked("vnd.apache.arrow.stream");
    /// `vnd.apache.thrift.binary`
    pub const APACHE_THRIFT_BINARY: crate::Name = crate::Name::new_unchecked("vnd.apache.thrift.binary");
    /// `vnd.apache.thrift.compact`
    pub const APACHE_THRIFT_COMPACT: crate::Name = crate::Name::new_unchecked("vnd.apache.thrift.compact");
    /// `vnd.apache.thrift.json`
    pub const APACHE_THRIFT_JSON: crate::Name = crate::Name::new_unchecked("vnd.apache.thrift.json");
    /// `vnd.api`
    pub const API: crate::Name = crate::Name::new_unchecked("vnd.api");
    /// `vnd.aplextor.warrp`
    pub const APLEXTOR_WARRP: crate::Name = crate::Name::new_unchecked("vnd.aplextor.warrp");
    /// `vnd.apothekende.reservation`
    pub const APOTHEKENDE_RESERVATION: crate::Name = crate::Name::new_unchecked("vnd.apothekende.reservation");
    /// `vnd.apple.installer`
    pub const APPLE_INSTALLER: crate::Name = crate::Name::new_unchecked("vnd.apple.installer");
    /// `vnd.apple.keynote`
    pub const APPLE_KEYNOTE: crate::Name = crate::Name::new_unchecked("vnd.apple.keynote");
    /// `vnd.apple.mpegurl`
    pub const APPLE_MPEGURL: crate::Name = crate::Name::new_unchecked("vnd.apple.mpegurl");
    /// `vnd.apple.numbers`
    pub const APPLE_NUMBERS: crate::Name = crate::Name::new_unchecked("vnd.apple.numbers");
    /// `vnd.apple.pages`
    pub const APPLE_PAGES: crate::Name = crate::Name::new_unchecked("vnd.apple.pages");
    /// `vnd.arastra.swi`
    pub const ARASTRA_SWI: crate::Name = crate::Name::new_unchecked("vnd.arastra.swi");
    /// `vnd.aristanetworks.swi`
    pub const ARISTANETWORKS_SWI: crate::Name = crate::Name::new_unchecked("vnd.aristanetworks.swi");
    /// `vnd.artisan`
    pub const ARTISAN: crate::Name = crate::Name::new_unchecked("vnd.artisan");
    /// `vnd.artsquare`
    pub const ARTSQUARE: crate::Name = crate::Name::new_unchecked("vnd.artsquare");
    /// `vnd.ascii-art`
    pub const ASCII_ART: crate::Name = crate::Name::new_unchecked("vnd.ascii-art");
    /// `vnd.astraea-software.iota`
    pub const ASTRAEA_SOFTWARE_IOTA: crate::Name = crate::Name::new_unchecked("vnd.astraea-software.iota");
    /// `vnd.audiograph`
    pub const AUDIOGRAPH: crate::Name = crate::Name::new_unchecked("vnd.audiograph");
    /// `vnd.audiokoz`
    pub const AUDIOKOZ: crate::Name = crate::Name::new_unchecked("vnd.audiokoz");
    /// `vnd.autopackage`
    pub const AUTOPACKAGE: crate::Name = crate::Name::new_unchecked("vnd.autopackage");
    /// `vnd.avalon`
    pub const AVALON: crate::Name = crate::Name::new_unchecked("vnd.avalon");
    /// `vnd.avistar`
    pub const AVISTAR: crate::Name = crate::Name::new_unchecked("vnd.avistar");
    /// `vnd.balsamiq.bmml`
    pub const BALSAMIQ_BMML: crate::Name = crate::Name::new_unchecked("vnd.balsamiq.bmml");
    /// `vnd.balsamiq.bmpr`
    pub const BALSAMIQ_BMPR: crate::Name = crate::Name::new_unchecked("vnd.balsamiq.bmpr");
    /// `vnd.banana-accounting`
    pub const BANANA_ACCOUNTING: crate::Name = crate::Name::new_unchecked("vnd.banana-accounting");
    /// `vnd.bbf.usp.error`
    pub const BBF_USP_ERROR: crate::Name = crate::Name::new_unchecked("vnd.bbf.usp.error");
    /// `vnd.bbf.usp.msg`
    pub const BBF_USP_MSG: crate::Name = crate::Name::new_unchecked("vnd.bbf.usp.msg");
    /// `vnd.bekitzur-stech`
    pub const BEKITZUR_STECH: crate::Name = crate::Name::new_unchecked("vnd.bekitzur-stech");
    /// `vnd.bint.med-content`
    pub const BINT_MED_CONTENT: crate::Name = crate::Name::new_unchecked("vnd.bint.med-content");
    /// `vnd.bint.med-plus`
    pub const BINT_MED_PLUS: crate::Name = crate::Name::new_unchecked("vnd.bint.med-plus");
    /// `vnd.biopax.rdf`
    pub const BIOPAX_RDF: crate::Name = crate::Name::new_unchecked("vnd.biopax.rdf");
    /// `vnd.blink-idb-value-wrapper`
    pub const BLINK_IDB_VALUE_WRAPPER: crate::Name = crate::Name::new_unchecked("vnd.blink-idb-value-wrapper");
    /// `vnd.blueice.multipass`
    pub const BLUEICE_MULTIPASS: crate::Name = crate::Name::new_unchecked("vnd.blueice.multipass");
    /// `vnd.bluetooth.ep.oob`
    pub const BLUETOOTH_EP_OOB: crate::Name = crate::Name::new_unchecked("vnd.bluetooth.ep.oob");
    /// `vnd.bluetooth.le.oob`
    pub const BLUETOOTH_LE_OOB: crate::Name = crate::Name::new_unchecked("vnd.bluetooth.le.oob");
    /// `vnd.bmi`
    pub const BMI: crate::Name = crate::Name::new_unchecked("vnd.bmi");
    /// `vnd.bpf`
    pub const BPF: crate::Name = crate::Name::new_unchecked("vnd.bpf");
    /// `vnd.bpf3`
    pub const BPF3: crate::Name = crate::Name::new_unchecked("vnd.bpf3");
    /// `vnd.businessobjects`
    pub const BUSINESSOBJECTS: crate::Name = crate::Name::new_unchecked("vnd.businessobjects");
    /// `vnd.byu.uapi`
    pub const BYU_UAPI: crate::Name = crate::Name::new_unchecked("vnd.byu.uapi");
    /// `vnd.cab-jscript`
    pub const CAB_JSCRIPT: crate::Name = crate::Name::new_unchecked("vnd.cab-jscript");
    /// `vnd.canon-cpdl`
    pub const CANON_CPDL: crate::Name = crate::Name::new_unchecked("vnd.canon-cpdl");
    /// `vnd.canon-lips`
    pub const CANON_LIPS: crate::Name = crate::Name::new_unchecked("vnd.canon-lips");
    /// `vnd.capasystems-pg`
    pub const CAPASYSTEMS_PG: crate::Name = crate::Name::new_unchecked("vnd.capasystems-pg");
    /// `vnd.CCTV`
    pub const CCTV: crate::Name = crate::Name::new_unchecked("vnd.CCTV");
    /// `vnd.CELP`
    pub const CELP: crate::Name = crate::Name::new_unchecked("vnd.CELP");
    /// `vnd.cendio.thinlinc.clientconf`
    pub const CENDIO_THINLINC_CLIENTCONF: crate::Name = crate::Name::new_unchecked("vnd.cendio.thinlinc.clientconf");
    /// `vnd.century-systems.tcp_stream`
    pub const CENTURY_SYSTEMS_TCP_STREAM: crate::Name = crate::Name::new_unchecked("vnd.century-systems.tcp_stream");
    /// `vnd.chemdraw`
    pub const CHEMDRAW: crate::Name = crate::Name::new_unchecked("vnd.chemdraw");
    /// `vnd.chess-pgn`
    pub const CHESS_PGN: crate::Name = crate::Name::new_unchecked("vnd.chess-pgn");
    /// `vnd.chipnuts.karaoke-mmd`
    pub const CHIPNUTS_KARAOKE_MMD: crate::Name = crate::Name::new_unchecked("vnd.chipnuts.karaoke-mmd");
    /// `vnd.ciedi`
    pub const CIEDI: crate::Name = crate::Name::new_unchecked("vnd.ciedi");
    /// `vnd.cinderella`
    pub const CINDERELLA: crate::Name = crate::Name::new_unchecked("vnd.cinderella");
    /// `vnd.cirpack.isdn-ext`
    pub const CIRPACK_ISDN_EXT: crate::Name = crate::Name::new_unchecked("vnd.cirpack.isdn-ext");
    /// `vnd.cisco.nse`
    pub const CISCO_NSE: crate::Name = crate::Name::new_unchecked("vnd.cisco.nse");
    /// `vnd.citationstyles.style`
    pub const CITATIONSTYLES_STYLE: crate::Name = crate::Name::new_unchecked("vnd.citationstyles.style");
    /// `vnd.claymore`
    pub const CLAYMORE: crate::Name = crate::Name::new_unchecked("vnd.claymore");
    /// `vnd.cloanto.rp9`
    pub const CLOANTO_RP9: crate::Name = crate::Name::new_unchecked("vnd.cloanto.rp9");
    /// `vnd.clonk.c4group`
    pub const CLONK_C4GROUP: crate::Name = crate::Name::new_unchecked("vnd.clonk.c4group");
    /// `vnd.cluetrust.cartomobile-config`
    pub const CLUETRUST_CARTOMOBILE_CONFIG: crate::Name = crate::Name::new_unchecked("vnd.cluetrust.cartomobile-config");
    /// `vnd.cluetrust.cartomobile-config-pkg`
    pub const CLUETRUST_CARTOMOBILE_CONFIG_PKG: crate::Name = crate::Name::new_unchecked("vnd.cluetrust.cartomobile-config-pkg");
    /// `vnd.cmles.radio-events`
    pub const CMLES_RADIO_EVENTS: crate::Name = crate::Name::new_unchecked("vnd.cmles.radio-events");
    /// `vnd.cns.anp1`
    pub const CNS_ANP1: crate::Name = crate::Name::new_unchecked("vnd.cns.anp1");
    /// `vnd.cns.inf1`
    pub const CNS_INF1: crate::Name = crate::Name::new_unchecked("vnd.cns.inf1");
    /// `vnd.cns.inf2`
    pub const CNS_INF2: crate::Name = crate::Name::new_unchecked("vnd.cns.inf2");
    /// `vnd.coffeescript`
    pub const COFFEESCRIPT: crate::Name = crate::Name::new_unchecked("vnd.coffeescript");
    /// `vnd.collabio.xodocuments.document`
    pub const COLLABIO_XODOCUMENTS_DOCUMENT: crate::Name = crate::Name::new_unchecked("vnd.collabio.xodocuments.document");
    /// `vnd.collabio.xodocuments.document-template`
    pub const COLLABIO_XODOCUMENTS_DOCUMENT_TEMPLATE: crate::Name = crate::Name::new_unchecked("vnd.collabio.xodocuments.document-template");
    /// `vnd.collabio.xodocuments.presentation`
    pub const COLLABIO_XODOCUMENTS_PRESENTATION: crate::Name = crate::Name::new_unchecked("vnd.collabio.xodocuments.presentation");
    /// `vnd.collabio.xodocuments.presentation-template`
    pub const COLLABIO_XODOCUMENTS_PRESENTATION_TEMPLATE: crate::Name = crate::Name::new_unchecked("vnd.collabio.xodocuments.presentation-template");
    /// `vnd.collabio.xodocuments.spreadsheet`
    pub const COLLABIO_XODOCUMENTS_SPREADSHEET: crate::Name = crate::Name::new_unchecked("vnd.collabio.xodocuments.spreadsheet");
    /// `vnd.collabio.xodocuments.spreadsheet-template`
    pub const COLLABIO_XODOCUMENTS_SPREADSHEET_TEMPLATE: crate::Name = crate::Name::new_unchecked("vnd.collabio.xodocuments.spreadsheet-template");
    /// `vnd.collada`
    pub const COLLADA: crate::Name = crate::Name::new_unchecked("vnd.collada");
    /// `vnd.collection`
    pub const COLLECTION: crate::Name = crate::Name::new_unchecked("vnd.collection");
    /// `vnd.collection.doc`
    pub const COLLECTION_DOC: crate::Name = crate::Name::new_unchecked("vnd.collection.doc");
    /// `vnd.collection.next`
    pub const COLLECTION_NEXT: crate::Name = crate::Name::new_unchecked("vnd.collection.next");
    /// `vnd.comicbook`
    pub const COMICBOOK: crate::Name = crate::Name::new_unchecked("vnd.comicbook");
    /// `vnd.comicbook-rar`
    pub const COMICBOOK_RAR: crate::Name = crate::Name::new_unchecked("vnd.comicbook-rar");
    /// `vnd.commerce-battelle`
    pub const COMMERCE_BATTELLE: crate::Name = crate::Name::new_unchecked("vnd.commerce-battelle");
    /// `vnd.commonspace`
    pub const COMMONSPACE: crate::Name = crate::Name::new_unchecked("vnd.commonspace");
    /// `vnd.contact.cmsg`
    pub const CONTACT_CMSG: crate::Name = crate::Name::new_unchecked("vnd.contact.cmsg");
    /// `vnd.coreos.ignition`
    pub const COREOS_IGNITION: crate::Name = crate::Name::new_unchecked("vnd.coreos.ignition");
    /// `vnd.cosmocaller`
    pub const COSMOCALLER: crate::Name = crate::Name::new_unchecked("vnd.cosmocaller");
    /// `vnd.crick.clicker`
    pub const CRICK_CLICKER: crate::Name = crate::Name::new_unchecked("vnd.crick.clicker");
    /// `vnd.crick.clicker.keyboard`
    pub const CRICK_CLICKER_KEYBOARD: crate::Name = crate::Name::new_unchecked("vnd.crick.clicker.keyboard");
    /// `vnd.crick.clicker.palette`
    pub const CRICK_CLICKER_PALETTE: crate::Name = crate::Name::new_unchecked("vnd.crick.clicker.palette");
    /// `vnd.crick.clicker.template`
    pub const CRICK_CLICKER_TEMPLATE: crate::Name = crate::Name::new_unchecked("vnd.crick.clicker.template");
    /// `vnd.crick.clicker.wordbank`
    pub const CRICK_CLICKER_WORDBANK: crate::Name = crate::Name::new_unchecked("vnd.crick.clicker.wordbank");
    /// `vnd.criticaltools.wbs`
    pub const CRITICALTOOLS_WBS: crate::Name = crate::Name::new_unchecked("vnd.criticaltools.wbs");
    /// `vnd.cryptii.pipe`
    pub const CRYPTII_PIPE: crate::Name = crate::Name::new_unchecked("vnd.cryptii.pipe");
    /// `vnd.crypto-shade-file`
    pub const CRYPTO_SHADE_FILE: crate::Name = crate::Name::new_unchecked("vnd.crypto-shade-file");
    /// `vnd.cryptomator.encrypted`
    pub const CRYPTOMATOR_ENCRYPTED: crate::Name = crate::Name::new_unchecked("vnd.cryptomator.encrypted");
    /// `vnd.cryptomator.vault`
    pub const CRYPTOMATOR_VAULT: crate::Name = crate::Name::new_unchecked("vnd.cryptomator.vault");
    /// `vnd.ctc-posml`
    pub const CTC_POSML: crate::Name = crate::Name::new_unchecked("vnd.ctc-posml");
    /// `vnd.ctct.ws`
    pub const CTCT_WS: crate::Name = crate::Name::new_unchecked("vnd.ctct.ws");
    /// `vnd.cups-pdf`
    pub const CUPS_PDF: crate::Name = crate::Name::new_unchecked("vnd.cups-pdf");
    /// `vnd.cups-postscript`
    pub const CUPS_POSTSCRIPT: crate::Name = crate::Name::new_unchecked("vnd.cups-postscript");
    /// `vnd.cups-ppd`
    pub const CUPS_PPD: crate::Name = crate::Name::new_unchecked("vnd.cups-ppd");
    /// `vnd.cups-raster`
    pub const CUPS_RASTER: crate::Name = crate::Name::new_unchecked("vnd.cups-raster");
    /// `vnd.cups-raw`
    pub const CUPS_RAW: crate::Name = crate::Name::new_unchecked("vnd.cups-raw");
    /// `vnd.curl`
    pub const CURL: crate::Name = crate::Name::new_unchecked("vnd.curl");
    /// `vnd.cyan.dean.root`
    pub const CYAN_DEAN_ROOT: crate::Name = crate::Name::new_unchecked("vnd.cyan.dean.root");
    /// `vnd.cybank`
    pub const CYBANK: crate::Name = crate::Name::new_unchecked("vnd.cybank");
    /// `vnd.cyclonedx`
    pub const CYCLONEDX: crate::Name = crate::Name::new_unchecked("vnd.cyclonedx");
    /// `vnd.d2l.coursepackage1p0`
    pub const D2L_COURSEPACKAGE1P0: crate::Name = crate::Name::new_unchecked("vnd.d2l.coursepackage1p0");
    /// `vnd.d3m-dataset`
    pub const D3M_DATASET: crate::Name = crate::Name::new_unchecked("vnd.d3m-dataset");
    /// `vnd.d3m-problem`
    pub const D3M_PROBLEM: crate::Name = crate::Name::new_unchecked("vnd.d3m-problem");
    /// `vnd.dart`
    pub const DART: crate::Name = crate::Name::new_unchecked("vnd.dart");
    /// `vnd.data-vision.rdz`
    pub const DATA_VISION_RDZ: crate::Name = crate::Name::new_unchecked("vnd.data-vision.rdz");
    /// `vnd.datapackage`
    pub const DATAPACKAGE: crate::Name = crate::Name::new_unchecked("vnd.datapackage");
    /// `vnd.dataresource`
    pub const DATARESOURCE: crate::Name = crate::Name::new_unchecked("vnd.dataresource");
    /// `vnd.dbf`
    pub const DBF: crate::Name = crate::Name::new_unchecked("vnd.dbf");
    /// `vnd.debian.binary-package`
    pub const DEBIAN_BINARY_PACKAGE: crate::Name = crate::Name::new_unchecked("vnd.debian.binary-package");
    /// `vnd.debian.copyright`
    pub const DEBIAN_COPYRIGHT: crate::Name = crate::Name::new_unchecked("vnd.debian.copyright");
    /// `vnd.dece.audio`
    pub const DECE_AUDIO: crate::Name = crate::Name::new_unchecked("vnd.dece.audio");
    /// `vnd.dece.data`
    pub const DECE_DATA: crate::Name = crate::Name::new_unchecked("vnd.dece.data");
    /// `vnd.dece.graphic`
    pub const DECE_GRAPHIC: crate::Name = crate::Name::new_unchecked("vnd.dece.graphic");
    /// `vnd.dece.hd`
    pub const DECE_HD: crate::Name = crate::Name::new_unchecked("vnd.dece.hd");
    /// `vnd.dece.mobile`
    pub const DECE_MOBILE: crate::Name = crate::Name::new_unchecked("vnd.dece.mobile");
    /// `vnd.dece.mp4`
    pub const DECE_MP4: crate::Name = crate::Name::new_unchecked("vnd.dece.mp4");
    /// `vnd.dece.pd`
    pub const DECE_PD: crate::Name = crate::Name::new_unchecked("vnd.dece.pd");
    /// `vnd.dece.sd`
    pub const DECE_SD: crate::Name = crate::Name::new_unchecked("vnd.dece.sd");
    /// `vnd.dece.ttml`
    pub const DECE_TTML: crate::Name = crate::Name::new_unchecked("vnd.dece.ttml");
    /// `vnd.dece.unspecified`
    pub const DECE_UNSPECIFIED: crate::Name = crate::Name::new_unchecked("vnd.dece.unspecified");
    /// `vnd.dece.video`
    pub const DECE_VIDEO: crate::Name = crate::Name::new_unchecked("vnd.dece.video");
    /// `vnd.dece.zip`
    pub const DECE_ZIP: crate::Name = crate::Name::new_unchecked("vnd.dece.zip");
    /// `vnd.denovo.fcselayout-link`
    pub const DENOVO_FCSELAYOUT_LINK: crate::Name = crate::Name::new_unchecked("vnd.denovo.fcselayout-link");
    /// `vnd.desmume.movie`
    pub const DESMUME_MOVIE: crate::Name = crate::Name::new_unchecked("vnd.desmume.movie");
    /// `vnd.digital-winds`
    pub const DIGITAL_WINDS: crate::Name = crate::Name::new_unchecked("vnd.digital-winds");
    /// `vnd.dir-bi.plate-dl-nosuffix`
    pub const DIR_BI_PLATE_DL_NOSUFFIX: crate::Name = crate::Name::new_unchecked("vnd.dir-bi.plate-dl-nosuffix");
    /// `vnd.directv.mpeg`
    pub const DIRECTV_MPEG: crate::Name = crate::Name::new_unchecked("vnd.directv.mpeg");
    /// `vnd.directv.mpeg-tts`
    pub const DIRECTV_MPEG_TTS: crate::Name = crate::Name::new_unchecked("vnd.directv.mpeg-tts");
    /// `vnd.djvu`
    pub const DJVU: crate::Name = crate::Name::new_unchecked("vnd.djvu");
    /// `vnd.dlna.adts`
    pub const DLNA_ADTS: crate::Name = crate::Name::new_unchecked("vnd.dlna.adts");
    /// `vnd.dlna.mpeg-tts`
    pub const DLNA_MPEG_TTS: crate::Name = crate::Name::new_unchecked("vnd.dlna.mpeg-tts");
    /// `vnd.dm.delegation`
    pub const DM_DELEGATION: crate::Name = crate::Name::new_unchecked("vnd.dm.delegation");
    /// `vnd.DMClientScript`
    pub const DMCLIENT_SCRIPT: crate::Name = crate::Name::new_unchecked("vnd.DMClientScript");
    /// `vnd.dna`
    pub const DNA: crate::Name = crate::Name::new_unchecked("vnd.dna");
    /// `vnd.document`
    pub const DOCUMENT: crate::Name = crate::Name::new_unchecked("vnd.document");
    /// `vnd.dolby.heaac.1`
    pub const DOLBY_HEAAC_1: crate::Name = crate::Name::new_unchecked("vnd.dolby.heaac.1");
    /// `vnd.dolby.heaac.2`
    pub const DOLBY_HEAAC_2: crate::Name = crate::Name::new_unchecked("vnd.dolby.heaac.2");
    /// `vnd.dolby.mlp`
    pub const DOLBY_MLP: crate::Name = crate::Name::new_unchecked("vnd.dolby.mlp");
    /// `vnd.dolby.mobile.1`
    pub const DOLBY_MOBILE_1: crate::Name = crate::Name::new_unchecked("vnd.dolby.mobile.1");
    /// `vnd.dolby.mobile.2`
    pub const DOLBY_MOBILE_2: crate::Name = crate::Name::new_unchecked("vnd.dolby.mobile.2");
    /// `vnd.dolby.mps`
    pub const DOLBY_MPS: crate::Name = crate::Name::new_unchecked("vnd.dolby.mps");
    /// `vnd.dolby.pl2`
    pub const DOLBY_PL2: crate::Name = crate::Name::new_unchecked("vnd.dolby.pl2");
    /// `vnd.dolby.pl2x`
    pub const DOLBY_PL2X: crate::Name = crate::Name::new_unchecked("vnd.dolby.pl2x");
    /// `vnd.dolby.pl2z`
    pub const DOLBY_PL2Z: crate::Name = crate::Name::new_unchecked("vnd.dolby.pl2z");
    /// `vnd.dolby.pulse.1`
    pub const DOLBY_PULSE_1: crate::Name = crate::Name::new_unchecked("vnd.dolby.pulse.1");
    /// `vnd.doremir.scorecloud-binary-document`
    pub const DOREMIR_SCORECLOUD_BINARY_DOCUMENT: crate::Name = crate::Name::new_unchecked("vnd.doremir.scorecloud-binary-document");
    /// `vnd.dpgraph`
    pub const DPGRAPH: crate::Name = crate::Name::new_unchecked("vnd.dpgraph");
    /// `vnd.dra`
    pub const DRA: crate::Name = crate::Name::new_unchecked("vnd.dra");
    /// `vnd.dreamfactory`
    pub const DREAMFACTORY: crate::Name = crate::Name::new_unchecked("vnd.dreamfactory");
    /// `vnd.drive`
    pub const DRIVE: crate::Name = crate::Name::new_unchecked("vnd.drive");
    /// `vnd.dtg.local`
    pub const DTG_LOCAL: crate::Name = crate::Name::new_unchecked("vnd.dtg.local");
    /// `vnd.dtg.local.flash`
    pub const DTG_LOCAL_FLASH: crate::Name = crate::Name::new_unchecked("vnd.dtg.local.flash");
    /// `vnd.dtg.local.html`
    pub const DTG_LOCAL_HTML: crate::Name = crate::Name::new_unchecked("vnd.dtg.local.html");
    /// `vnd.dts`
    pub const DTS: crate::Name = crate::Name::new_unchecked("vnd.dts");
    /// `vnd.dts.hd`
    pub const DTS_HD: crate::Name = crate::Name::new_unchecked("vnd.dts.hd");
    /// `vnd.dts.uhd`
    pub const DTS_UHD: crate::Name = crate::Name::new_unchecked("vnd.dts.uhd");
    /// `vnd.dvb.ait`
    pub const DVB_AIT: crate::Name = crate::Name::new_unchecked("vnd.dvb.ait");
    /// `vnd.dvb.dvbisl`
    pub const DVB_DVBISL: crate::Name = crate::Name::new_unchecked("vnd.dvb.dvbisl");
    /// `vnd.dvb.dvbj`
    pub const DVB_DVBJ: crate::Name = crate::Name::new_unchecked("vnd.dvb.dvbj");
    /// `vnd.dvb.esgcontainer`
    pub const DVB_ESGCONTAINER: crate::Name = crate::Name::new_unchecked("vnd.dvb.esgcontainer");
    /// `vnd.dvb.file`
    pub const DVB_FILE: crate::Name = crate::Name::new_unchecked("vnd.dvb.file");
    /// `vnd.dvb.ipdcdftnotifaccess`
    pub const DVB_IPDCDFTNOTIFACCESS: crate::Name = crate::Name::new_unchecked("vnd.dvb.ipdcdftnotifaccess");
    /// `vnd.dvb.ipdcesgaccess`
    pub const DVB_IPDCESGACCESS: crate::Name = crate::Name::new_unchecked("vnd.dvb.ipdcesgaccess");
    /// `vnd.dvb.ipdcesgaccess2`
    pub const DVB_IPDCESGACCESS2: crate::Name = crate::Name::new_unchecked("vnd.dvb.ipdcesgaccess2");
    /// `vnd.dvb.ipdcesgpdd`
    pub const DVB_IPDCESGPDD: crate::Name = crate::Name::new_unchecked("vnd.dvb.ipdcesgpdd");
    /// `vnd.dvb.ipdcroaming`
    pub const DVB_IPDCROAMING: crate::Name = crate::Name::new_unchecked("vnd.dvb.ipdcroaming");
    /// `vnd.dvb.iptv.alfec-base`
    pub const DVB_IPTV_ALFEC_BASE: crate::Name = crate::Name::new_unchecked("vnd.dvb.iptv.alfec-base");
    /// `vnd.dvb.iptv.alfec-enhancement`
    pub const DVB_IPTV_ALFEC_ENHANCEMENT: crate::Name = crate::Name::new_unchecked("vnd.dvb.iptv.alfec-enhancement");
    /// `vnd.dvb.notif-aggregate-root`
    pub const DVB_NOTIF_AGGREGATE_ROOT: crate::Name = crate::Name::new_unchecked("vnd.dvb.notif-aggregate-root");
    /// `vnd.dvb.notif-container`
    pub const DVB_NOTIF_CONTAINER: crate::Name = crate::Name::new_unchecked("vnd.dvb.notif-container");
    /// `vnd.dvb.notif-generic`
    pub const DVB_NOTIF_GENERIC: crate::Name = crate::Name::new_unchecked("vnd.dvb.notif-generic");
    /// `vnd.dvb.notif-ia-msglist`
    pub const DVB_NOTIF_IA_MSGLIST: crate::Name = crate::Name::new_unchecked("vnd.dvb.notif-ia-msglist");
    /// `vnd.dvb.notif-ia-registration-request`
    pub const DVB_NOTIF_IA_REGISTRATION_REQUEST: crate::Name = crate::Name::new_unchecked("vnd.dvb.notif-ia-registration-request");
    /// `vnd.dvb.notif-ia-registration-response`
    pub const DVB_NOTIF_IA_REGISTRATION_RESPONSE: crate::Name = crate::Name::new_unchecked("vnd.dvb.notif-ia-registration-response");
    /// `vnd.dvb.notif-init`
    pub const DVB_NOTIF_INIT: crate::Name = crate::Name::new_unchecked("vnd.dvb.notif-init");
    /// `vnd.dvb.pfr`
    pub const DVB_PFR: crate::Name = crate::Name::new_unchecked("vnd.dvb.pfr");
    /// `vnd.dvb.service`
    pub const DVB_SERVICE: crate::Name = crate::Name::new_unchecked("vnd.dvb.service");
    /// `vnd.dvb.subtitle`
    pub const DVB_SUBTITLE: crate::Name = crate::Name::new_unchecked("vnd.dvb.subtitle");
    /// `vnd.dwf`
    pub const DWF: crate::Name = crate::Name::new_unchecked("vnd.dwf");
    /// `vnd.dwg`
    pub const DWG: crate::Name = crate::Name::new_unchecked("vnd.dwg");
    /// `vnd.dxf`
    pub const DXF: crate::Name = crate::Name::new_unchecked("vnd.dxf");
    /// `vnd.dxr`
    pub const DXR: crate::Name = crate::Name::new_unchecked("vnd.dxr");
    /// `vnd.dynageo`
    pub const DYNAGEO: crate::Name = crate::Name::new_unchecked("vnd.dynageo");
    /// `vnd.dzr`
    pub const DZR: crate::Name = crate::Name::new_unchecked("vnd.dzr");
    /// `vnd.easykaraoke.cdgdownload`
    pub const EASYKARAOKE_CDGDOWNLOAD: crate::Name = crate::Name::new_unchecked("vnd.easykaraoke.cdgdownload");
    /// `vnd.ecdis-update`
    pub const ECDIS_UPDATE: crate::Name = crate::Name::new_unchecked("vnd.ecdis-update");
    /// `vnd.ecip.rlp`
    pub const ECIP_RLP: crate::Name = crate::Name::new_unchecked("vnd.ecip.rlp");
    /// `vnd.eclipse.ditto`
    pub const ECLIPSE_DITTO: crate::Name = crate::Name::new_unchecked("vnd.eclipse.ditto");
    /// `vnd.ecowin.chart`
    pub const ECOWIN_CHART: crate::Name = crate::Name::new_unchecked("vnd.ecowin.chart");
    /// `vnd.ecowin.filerequest`
    pub const ECOWIN_FILEREQUEST: crate::Name = crate::Name::new_unchecked("vnd.ecowin.filerequest");
    /// `vnd.ecowin.fileupdate`
    pub const ECOWIN_FILEUPDATE: crate::Name = crate::Name::new_unchecked("vnd.ecowin.fileupdate");
    /// `vnd.ecowin.series`
    pub const ECOWIN_SERIES: crate::Name = crate::Name::new_unchecked("vnd.ecowin.series");
    /// `vnd.ecowin.seriesrequest`
    pub const ECOWIN_SERIESREQUEST: crate::Name = crate::Name::new_unchecked("vnd.ecowin.seriesrequest");
    /// `vnd.ecowin.seriesupdate`
    pub const ECOWIN_SERIESUPDATE: crate::Name = crate::Name::new_unchecked("vnd.ecowin.seriesupdate");
    /// `vnd.efi.img`
    pub const EFI_IMG: crate::Name = crate::Name::new_unchecked("vnd.efi.img");
    /// `vnd.efi.iso`
    pub const EFI_ISO: crate::Name = crate::Name::new_unchecked("vnd.efi.iso");
    /// `vnd.emclient.accessrequest`
    pub const EMCLIENT_ACCESSREQUEST: crate::Name = crate::Name::new_unchecked("vnd.emclient.accessrequest");
    /// `vnd.enliven`
    pub const ENLIVEN: crate::Name = crate::Name::new_unchecked("vnd.enliven");
    /// `vnd.enphase.envoy`
    pub const ENPHASE_ENVOY: crate::Name = crate::Name::new_unchecked("vnd.enphase.envoy");
    /// `vnd.eprints.data`
    pub const EPRINTS_DATA: crate::Name = crate::Name::new_unchecked("vnd.eprints.data");
    /// `vnd.epson.esf`
    pub const EPSON_ESF: crate::Name = crate::Name::new_unchecked("vnd.epson.esf");
    /// `vnd.epson.msf`
    pub const EPSON_MSF: crate::Name = crate::Name::new_unchecked("vnd.epson.msf");
    /// `vnd.epson.quickanime`
    pub const EPSON_QUICKANIME: crate::Name = crate::Name::new_unchecked("vnd.epson.quickanime");
    /// `vnd.epson.salt`
    pub const EPSON_SALT: crate::Name = crate::Name::new_unchecked("vnd.epson.salt");
    /// `vnd.epson.ssf`
    pub const EPSON_SSF: crate::Name = crate::Name::new_unchecked("vnd.epson.ssf");
    /// `vnd.ericsson.quickcall`
    pub const ERICSSON_QUICKCALL: crate::Name = crate::Name::new_unchecked("vnd.ericsson.quickcall");
    /// `vnd.esmertec.theme-descriptor`
    pub const ESMERTEC_THEME_DESCRIPTOR: crate::Name = crate::Name::new_unchecked("vnd.esmertec.theme-descriptor");
    /// `vnd.espass-espass`
    pub const ESPASS_ESPASS: crate::Name = crate::Name::new_unchecked("vnd.espass-espass");
    /// `vnd.eszigno3`
    pub const ESZIGNO3: crate::Name = crate::Name::new_unchecked("vnd.eszigno3");
    /// `vnd.etsi.aoc`
    pub const ETSI_AOC: crate::Name = crate::Name::new_unchecked("vnd.etsi.aoc");
    /// `vnd.etsi.asic-e`
    pub const ETSI_ASIC_E: crate::Name = crate::Name::new_unchecked("vnd.etsi.asic-e");
    /// `vnd.etsi.asic-s`
    pub const ETSI_ASIC_S: crate::Name = crate::Name::new_unchecked("vnd.etsi.asic-s");
    /// `vnd.etsi.cug`
    pub const ETSI_CUG: crate::Name = crate::Name::new_unchecked("vnd.etsi.cug");
    /// `vnd.etsi.iptvcommand`
    pub const ETSI_IPTVCOMMAND: crate::Name = crate::Name::new_unchecked("vnd.etsi.iptvcommand");
    /// `vnd.etsi.iptvdiscovery`
    pub const ETSI_IPTVDISCOVERY: crate::Name = crate::Name::new_unchecked("vnd.etsi.iptvdiscovery");
    /// `vnd.etsi.iptvprofile`
    pub const ETSI_IPTVPROFILE: crate::Name = crate::Name::new_unchecked("vnd.etsi.iptvprofile");
    /// `vnd.etsi.iptvsad-bc`
    pub const ETSI_IPTVSAD_BC: crate::Name = crate::Name::new_unchecked("vnd.etsi.iptvsad-bc");
    /// `vnd.etsi.iptvsad-cod`
    pub const ETSI_IPTVSAD_COD: crate::Name = crate::Name::new_unchecked("vnd.etsi.iptvsad-cod");
    /// `vnd.etsi.iptvsad-npvr`
    pub const ETSI_IPTVSAD_NPVR: crate::Name = crate::Name::new_unchecked("vnd.etsi.iptvsad-npvr");
    /// `vnd.etsi.iptvservice`
    pub const ETSI_IPTVSERVICE: crate::Name = crate::Name::new_unchecked("vnd.etsi.iptvservice");
    /// `vnd.etsi.iptvsync`
    pub const ETSI_IPTVSYNC: crate::Name = crate::Name::new_unchecked("vnd.etsi.iptvsync");
    /// `vnd.etsi.iptvueprofile`
    pub const ETSI_IPTVUEPROFILE: crate::Name = crate::Name::new_unchecked("vnd.etsi.iptvueprofile");
    /// `vnd.etsi.mcid`
    pub const ETSI_MCID: crate::Name = crate::Name::new_unchecked("vnd.etsi.mcid");
    /// `vnd.etsi.mheg5`
    pub const ETSI_MHEG5: crate::Name = crate::Name::new_unchecked("vnd.etsi.mheg5");
    /// `vnd.etsi.overload-control-policy-dataset`
    pub const ETSI_OVERLOAD_CONTROL_POLICY_DATASET: crate::Name = crate::Name::new_unchecked("vnd.etsi.overload-control-policy-dataset");
    /// `vnd.etsi.pstn`
    pub const ETSI_PSTN: crate::Name = crate::Name::new_unchecked("vnd.etsi.pstn");
    /// `vnd.etsi.sci`
    pub const ETSI_SCI: crate::Name = crate::Name::new_unchecked("vnd.etsi.sci");
    /// `vnd.etsi.simservs`
    pub const ETSI_SIMSERVS: crate::Name = crate::Name::new_unchecked("vnd.etsi.simservs");
    /// `vnd.etsi.timestamp-token`
    pub const ETSI_TIMESTAMP_TOKEN: crate::Name = crate::Name::new_unchecked("vnd.etsi.timestamp-token");
    /// `vnd.etsi.tsl`
    pub const ETSI_TSL: crate::Name = crate::Name::new_unchecked("vnd.etsi.tsl");
    /// `vnd.etsi.tsl.der`
    pub const ETSI_TSL_DER: crate::Name = crate::Name::new_unchecked("vnd.etsi.tsl.der");
    /// `vnd.eu.kasparian.car`
    pub const EU_KASPARIAN_CAR: crate::Name = crate::Name::new_unchecked("vnd.eu.kasparian.car");
    /// `vnd.eudora.data`
    pub const EUDORA_DATA: crate::Name = crate::Name::new_unchecked("vnd.eudora.data");
    /// `vnd.everad.plj`
    pub const EVERAD_PLJ: crate::Name = crate::Name::new_unchecked("vnd.everad.plj");
    /// `vnd.evolv.ecig.profile`
    pub const EVOLV_ECIG_PROFILE: crate::Name = crate::Name::new_unchecked("vnd.evolv.ecig.profile");
    /// `vnd.evolv.ecig.settings`
    pub const EVOLV_ECIG_SETTINGS: crate::Name = crate::Name::new_unchecked("vnd.evolv.ecig.settings");
    /// `vnd.evolv.ecig.theme`
    pub const EVOLV_ECIG_THEME: crate::Name = crate::Name::new_unchecked("vnd.evolv.ecig.theme");
    /// `vnd.exstream-empower`
    pub const EXSTREAM_EMPOWER: crate::Name = crate::Name::new_unchecked("vnd.exstream-empower");
    /// `vnd.exstream-package`
    pub const EXSTREAM_PACKAGE: crate::Name = crate::Name::new_unchecked("vnd.exstream-package");
    /// `vnd.ezpix-album`
    pub const EZPIX_ALBUM: crate::Name = crate::Name::new_unchecked("vnd.ezpix-album");
    /// `vnd.ezpix-package`
    pub const EZPIX_PACKAGE: crate::Name = crate::Name::new_unchecked("vnd.ezpix-package");
    /// `vnd.f-secure.mobile`
    pub const F_SECURE_MOBILE: crate::Name = crate::Name::new_unchecked("vnd.f-secure.mobile");
    /// `vnd.familysearch.gedcom`
    pub const FAMILYSEARCH_GEDCOM: crate::Name = crate::Name::new_unchecked("vnd.familysearch.gedcom");
    /// `vnd.fastbidsheet`
    pub const FASTBIDSHEET: crate::Name = crate::Name::new_unchecked("vnd.fastbidsheet");
    /// `vnd.fastcopy-disk-image`
    pub const FASTCOPY_DISK_IMAGE: crate::Name = crate::Name::new_unchecked("vnd.fastcopy-disk-image");
    /// `vnd.fdf`
    pub const FDF: crate::Name = crate::Name::new_unchecked("vnd.fdf");
    /// `vnd.fdsn.mseed`
    pub const FDSN_MSEED: crate::Name = crate::Name::new_unchecked("vnd.fdsn.mseed");
    /// `vnd.fdsn.seed`
    pub const FDSN_SEED: crate::Name = crate::Name::new_unchecked("vnd.fdsn.seed");
    /// `vnd.ffsns`
    pub const FFSNS: crate::Name = crate::Name::new_unchecked("vnd.ffsns");
    /// `vnd.ficlab.flb`
    pub const FICLAB_FLB: crate::Name = crate::Name::new_unchecked("vnd.ficlab.flb");
    /// `vnd.ficlab.flt`
    pub const FICLAB_FLT: crate::Name = crate::Name::new_unchecked("vnd.ficlab.flt");
    /// `vnd.filmit.zfc`
    pub const FILMIT_ZFC: crate::Name = crate::Name::new_unchecked("vnd.filmit.zfc");
    /// `vnd.fints`
    pub const FINTS: crate::Name = crate::Name::new_unchecked("vnd.fints");
    /// `vnd.firemonkeys.cloudcell`
    pub const FIREMONKEYS_CLOUDCELL: crate::Name = crate::Name::new_unchecked("vnd.firemonkeys.cloudcell");
    /// `vnd.flatland.3dml`
    pub const FLATLAND_3DML: crate::Name = crate::Name::new_unchecked("vnd.flatland.3dml");
    /// `vnd.FloGraphIt`
    pub const FLO_GRAPH_IT: crate::Name = crate::Name::new_unchecked("vnd.FloGraphIt");
    /// `vnd.fluxtime.clip`
    pub const FLUXTIME_CLIP: crate::Name = crate::Name::new_unchecked("vnd.fluxtime.clip");
    /// `vnd.fly`
    pub const FLY: crate::Name = crate::Name::new_unchecked("vnd.fly");
    /// `vnd.fmi.flexstor`
    pub const FMI_FLEXSTOR: crate::Name = crate::Name::new_unchecked("vnd.fmi.flexstor");
    /// `vnd.font-fontforge-sfd`
    pub const FONT_FONTFORGE_SFD: crate::Name = crate::Name::new_unchecked("vnd.font-fontforge-sfd");
    /// `vnd.fpx`
    pub const FPX: crate::Name = crate::Name::new_unchecked("vnd.fpx");
    /// `vnd.framemaker`
    pub const FRAMEMAKER: crate::Name = crate::Name::new_unchecked("vnd.framemaker");
    /// `vnd.frogans.fnc`
    pub const FROGANS_FNC: crate::Name = crate::Name::new_unchecked("vnd.frogans.fnc");
    /// `vnd.frogans.ltf`
    pub const FROGANS_LTF: crate::Name = crate::Name::new_unchecked("vnd.frogans.ltf");
    /// `vnd.fsc.weblaunch`
    pub const FSC_WEBLAUNCH: crate::Name = crate::Name::new_unchecked("vnd.fsc.weblaunch");
    /// `vnd.fst`
    pub const FST: crate::Name = crate::Name::new_unchecked("vnd.fst");
    /// `vnd.fujifilm.fb.docuworks`
    pub const FUJIFILM_FB_DOCUWORKS: crate::Name = crate::Name::new_unchecked("vnd.fujifilm.fb.docuworks");
    /// `vnd.fujifilm.fb.docuworks.binder`
    pub const FUJIFILM_FB_DOCUWORKS_BINDER: crate::Name = crate::Name::new_unchecked("vnd.fujifilm.fb.docuworks.binder");
    /// `vnd.fujifilm.fb.docuworks.container`
    pub const FUJIFILM_FB_DOCUWORKS_CONTAINER: crate::Name = crate::Name::new_unchecked("vnd.fujifilm.fb.docuworks.container");
    /// `vnd.fujifilm.fb.jfi`
    pub const FUJIFILM_FB_JFI: crate::Name = crate::Name::new_unchecked("vnd.fujifilm.fb.jfi");
    /// `vnd.fujitsu.oasys`
    pub const FUJITSU_OASYS: crate::Name = crate::Name::new_unchecked("vnd.fujitsu.oasys");
    /// `vnd.fujitsu.oasys2`
    pub const FUJITSU_OASYS2: crate::Name = crate::Name::new_unchecked("vnd.fujitsu.oasys2");
    /// `vnd.fujitsu.oasys3`
    pub const FUJITSU_OASYS3: crate::Name = crate::Name::new_unchecked("vnd.fujitsu.oasys3");
    /// `vnd.fujitsu.oasysgp`
    pub const FUJITSU_OASYSGP: crate::Name = crate::Name::new_unchecked("vnd.fujitsu.oasysgp");
    /// `vnd.fujitsu.oasysprs`
    pub const FUJITSU_OASYSPRS: crate::Name = crate::Name::new_unchecked("vnd.fujitsu.oasysprs");
    /// `vnd.fujixerox.ART-EX`
    pub const FUJIXEROX_ART_EX: crate::Name = crate::Name::new_unchecked("vnd.fujixerox.ART-EX");
    /// `vnd.fujixerox.ART4`
    pub const FUJIXEROX_ART4: crate::Name = crate::Name::new_unchecked("vnd.fujixerox.ART4");
    /// `vnd.fujixerox.ddd`
    pub const FUJIXEROX_DDD: crate::Name = crate::Name::new_unchecked("vnd.fujixerox.ddd");
    /// `vnd.fujixerox.docuworks`
    pub const FUJIXEROX_DOCUWORKS: crate::Name = crate::Name::new_unchecked("vnd.fujixerox.docuworks");
    /// `vnd.fujixerox.docuworks.binder`
    pub const FUJIXEROX_DOCUWORKS_BINDER: crate::Name = crate::Name::new_unchecked("vnd.fujixerox.docuworks.binder");
    /// `vnd.fujixerox.docuworks.container`
    pub const FUJIXEROX_DOCUWORKS_CONTAINER: crate::Name = crate::Name::new_unchecked("vnd.fujixerox.docuworks.container");
    /// `vnd.fujixerox.edmics-mmr`
    pub const FUJIXEROX_EDMICS_MMR: crate::Name = crate::Name::new_unchecked("vnd.fujixerox.edmics-mmr");
    /// `vnd.fujixerox.edmics-rlc`
    pub const FUJIXEROX_EDMICS_RLC: crate::Name = crate::Name::new_unchecked("vnd.fujixerox.edmics-rlc");
    /// `vnd.fujixerox.HBPL`
    pub const FUJIXEROX_HBPL: crate::Name = crate::Name::new_unchecked("vnd.fujixerox.HBPL");
    /// `vnd.fut-misnet`
    pub const FUT_MISNET: crate::Name = crate::Name::new_unchecked("vnd.fut-misnet");
    /// `vnd.futoin`
    pub const FUTOIN: crate::Name = crate::Name::new_unchecked("vnd.futoin");
    /// `vnd.fuzzysheet`
    pub const FUZZYSHEET: crate::Name = crate::Name::new_unchecked("vnd.fuzzysheet");
    /// `vnd.fvt`
    pub const FVT: crate::Name = crate::Name::new_unchecked("vnd.fvt");
    /// `vnd.gdl`
    pub const GDL: crate::Name = crate::Name::new_unchecked("vnd.gdl");
    /// `vnd.genomatix.tuxedo`
    pub const GENOMATIX_TUXEDO: crate::Name = crate::Name::new_unchecked("vnd.genomatix.tuxedo");
    /// `vnd.gentics.grd`
    pub const GENTICS_GRD: crate::Name = crate::Name::new_unchecked("vnd.gentics.grd");
    /// `vnd.geo`
    pub const GEO: crate::Name = crate::Name::new_unchecked("vnd.geo");
    /// `vnd.geocube`
    pub const GEOCUBE: crate::Name = crate::Name::new_unchecked("vnd.geocube");
    /// `vnd.geogebra.file`
    pub const GEOGEBRA_FILE: crate::Name = crate::Name::new_unchecked("vnd.geogebra.file");
    /// `vnd.geogebra.slides`
    pub const GEOGEBRA_SLIDES: crate::Name = crate::Name::new_unchecked("vnd.geogebra.slides");
    /// `vnd.geogebra.tool`
    pub const GEOGEBRA_TOOL: crate::Name = crate::Name::new_unchecked("vnd.geogebra.tool");
    /// `vnd.geometry-explorer`
    pub const GEOMETRY_EXPLORER: crate::Name = crate::Name::new_unchecked("vnd.geometry-explorer");
    /// `vnd.geonext`
    pub const GEONEXT: crate::Name = crate::Name::new_unchecked("vnd.geonext");
    /// `vnd.geoplan`
    pub const GEOPLAN: crate::Name = crate::Name::new_unchecked("vnd.geoplan");
    /// `vnd.geospace`
    pub const GEOSPACE: crate::Name = crate::Name::new_unchecked("vnd.geospace");
    /// `vnd.gerber`
    pub const GERBER: crate::Name = crate::Name::new_unchecked("vnd.gerber");
    /// `vnd.globalgraphics.pgb`
    pub const GLOBALGRAPHICS_PGB: crate::Name = crate::Name::new_unchecked("vnd.globalgraphics.pgb");
    /// `vnd.globalplatform.card-content-mgt`
    pub const GLOBALPLATFORM_CARD_CONTENT_MGT: crate::Name = crate::Name::new_unchecked("vnd.globalplatform.card-content-mgt");
    /// `vnd.globalplatform.card-content-mgt-response`
    pub const GLOBALPLATFORM_CARD_CONTENT_MGT_RESPONSE: crate::Name = crate::Name::new_unchecked("vnd.globalplatform.card-content-mgt-response");
    /// `vnd.gml`
    pub const GML: crate::Name = crate::Name::new_unchecked("vnd.gml");
    /// `vnd.gmx`
    pub const GMX: crate::Name = crate::Name::new_unchecked("vnd.gmx");
    /// `vnd.google-earth.kml`
    pub const GOOGLE_EARTH_KML: crate::Name = crate::Name::new_unchecked("vnd.google-earth.kml");
    /// `vnd.google-earth.kmz`
    pub const GOOGLE_EARTH_KMZ: crate::Name = crate::Name::new_unchecked("vnd.google-earth.kmz");
    /// `vnd.gov.sk.e-form`
    pub const GOV_SK_E_FORM: crate::Name = crate::Name::new_unchecked("vnd.gov.sk.e-form");
    /// `vnd.gov.sk.xmldatacontainer`
    pub const GOV_SK_XMLDATACONTAINER: crate::Name = crate::Name::new_unchecked("vnd.gov.sk.xmldatacontainer");
    /// `vnd.grafeq`
    pub const GRAFEQ: crate::Name = crate::Name::new_unchecked("vnd.grafeq");
    /// `vnd.graphviz`
    pub const GRAPHVIZ: crate::Name = crate::Name::new_unchecked("vnd.graphviz");
    /// `vnd.gridmp`
    pub const GRIDMP: crate::Name = crate::Name::new_unchecked("vnd.gridmp");
    /// `vnd.groove-account`
    pub const GROOVE_ACCOUNT: crate::Name = crate::Name::new_unchecked("vnd.groove-account");
    /// `vnd.groove-help`
    pub const GROOVE_HELP: crate::Name = crate::Name::new_unchecked("vnd.groove-help");
    /// `vnd.groove-identity-message`
    pub const GROOVE_IDENTITY_MESSAGE: crate::Name = crate::Name::new_unchecked("vnd.groove-identity-message");
    /// `vnd.groove-injector`
    pub const GROOVE_INJECTOR: crate::Name = crate::Name::new_unchecked("vnd.groove-injector");
    /// `vnd.groove-tool-message`
    pub const GROOVE_TOOL_MESSAGE: crate::Name = crate::Name::new_unchecked("vnd.groove-tool-message");
    /// `vnd.groove-tool-template`
    pub const GROOVE_TOOL_TEMPLATE: crate::Name = crate::Name::new_unchecked("vnd.groove-tool-template");
    /// `vnd.groove-vcard`
    pub const GROOVE_VCARD: crate::Name = crate::Name::new_unchecked("vnd.groove-vcard");
    /// `vnd.gs-gdl`
    pub const GS_GDL: crate::Name = crate::Name::new_unchecked("vnd.gs-gdl");
    /// `vnd.gtw`
    pub const GTW: crate::Name = crate::Name::new_unchecked("vnd.gtw");
    /// `vnd.hal`
    pub const HAL: crate::Name = crate::Name::new_unchecked("vnd.hal");
    /// `vnd.HandHeld-Entertainment`
    pub const HAND_HELD_ENTERTAINMENT: crate::Name = crate::Name::new_unchecked("vnd.HandHeld-Entertainment");
    /// `vnd.hans`
    pub const HANS: crate::Name = crate::Name::new_unchecked("vnd.hans");
    /// `vnd.hbci`
    pub const HBCI: crate::Name = crate::Name::new_unchecked("vnd.hbci");
    /// `vnd.hc`
    pub const HC: crate::Name = crate::Name::new_unchecked("vnd.hc");
    /// `vnd.hcl-bireports`
    pub const HCL_BIREPORTS: crate::Name = crate::Name::new_unchecked("vnd.hcl-bireports");
    /// `vnd.hdt`
    pub const HDT: crate::Name = crate::Name::new_unchecked("vnd.hdt");
    /// `vnd.heroku`
    pub const HEROKU: crate::Name = crate::Name::new_unchecked("vnd.heroku");
    /// `vnd.hgl`
    pub const HGL: crate::Name = crate::Name::new_unchecked("vnd.hgl");
    /// `vnd.hhe.lesson-player`
    pub const HHE_LESSON_PLAYER: crate::Name = crate::Name::new_unchecked("vnd.hhe.lesson-player");
    /// `vnd.hl7cda`
    pub const HL7CDA: crate::Name = crate::Name::new_unchecked("vnd.hl7cda");
    /// `vnd.hl7v2`
    pub const HL7V2: crate::Name = crate::Name::new_unchecked("vnd.hl7v2");
    /// `vnd.hns.audio`
    pub const HNS_AUDIO: crate::Name = crate::Name::new_unchecked("vnd.hns.audio");
    /// `vnd.hns.video`
    pub const HNS_VIDEO: crate::Name = crate::Name::new_unchecked("vnd.hns.video");
    /// `vnd.hp-HPGL`
    pub const HP_HPGL: crate::Name = crate::Name::new_unchecked("vnd.hp-HPGL");
    /// `vnd.hp-hpid`
    pub const HP_HPID: crate::Name = crate::Name::new_unchecked("vnd.hp-hpid");
    /// `vnd.hp-hps`
    pub const HP_HPS: crate::Name = crate::Name::new_unchecked("vnd.hp-hps");
    /// `vnd.hp-jlyt`
    pub const HP_JLYT: crate::Name = crate::Name::new_unchecked("vnd.hp-jlyt");
    /// `vnd.hp-PCL`
    pub const HP_PCL: crate::Name = crate::Name::new_unchecked("vnd.hp-PCL");
    /// `vnd.hp-PCLXL`
    pub const HP_PCLXL: crate::Name = crate::Name::new_unchecked("vnd.hp-PCLXL");
    /// `vnd.httphone`
    pub const HTTPHONE: crate::Name = crate::Name::new_unchecked("vnd.httphone");
    /// `vnd.hydrostatix.sof-data`
    pub const HYDROSTATIX_SOF_DATA: crate::Name = crate::Name::new_unchecked("vnd.hydrostatix.sof-data");
    /// `vnd.hyper`
    pub const HYPER: crate::Name = crate::Name::new_unchecked("vnd.hyper");
    /// `vnd.hyper-item`
    pub const HYPER_ITEM: crate::Name = crate::Name::new_unchecked("vnd.hyper-item");
    /// `vnd.hyperdrive`
    pub const HYPERDRIVE: crate::Name = crate::Name::new_unchecked("vnd.hyperdrive");
    /// `vnd.hzn-3d-crossword`
    pub const HZN_3D_CROSSWORD: crate::Name = crate::Name::new_unchecked("vnd.hzn-3d-crossword");
    /// `vnd.ibm.afplinedata`
    pub const IBM_AFPLINEDATA: crate::Name = crate::Name::new_unchecked("vnd.ibm.afplinedata");
    /// `vnd.ibm.electronic-media`
    pub const IBM_ELECTRONIC_MEDIA: crate::Name = crate::Name::new_unchecked("vnd.ibm.electronic-media");
    /// `vnd.ibm.MiniPay`
    pub const IBM_MINI_PAY: crate::Name = crate::Name::new_unchecked("vnd.ibm.MiniPay");
    /// `vnd.ibm.modcap`
    pub const IBM_MODCAP: crate::Name = crate::Name::new_unchecked("vnd.ibm.modcap");
    /// `vnd.ibm.rights-management`
    pub const IBM_RIGHTS_MANAGEMENT: crate::Name = crate::Name::new_unchecked("vnd.ibm.rights-management");
    /// `vnd.ibm.secure-container`
    pub const IBM_SECURE_CONTAINER: crate::Name = crate::Name::new_unchecked("vnd.ibm.secure-container");
    /// `vnd.iccprofile`
    pub const ICCPROFILE: crate::Name = crate::Name::new_unchecked("vnd.iccprofile");
    /// `vnd.ieee.1905`
    pub const IEEE_1905: crate::Name = crate::Name::new_unchecked("vnd.ieee.1905");
    /// `vnd.igloader`
    pub const IGLOADER: crate::Name = crate::Name::new_unchecked("vnd.igloader");
    /// `vnd.imagemeter.folder`
    pub const IMAGEMETER_FOLDER: crate::Name = crate::Name::new_unchecked("vnd.imagemeter.folder");
    /// `vnd.imagemeter.image`
    pub const IMAGEMETER_IMAGE: crate::Name = crate::Name::new_unchecked("vnd.imagemeter.image");
    /// `vnd.immervision-ivp`
    pub const IMMERVISION_IVP: crate::Name = crate::Name::new_unchecked("vnd.immervision-ivp");
    /// `vnd.immervision-ivu`
    pub const IMMERVISION_IVU: crate::Name = crate::Name::new_unchecked("vnd.immervision-ivu");
    /// `vnd.ims.imsccv1p1`
    pub const IMS_IMSCCV1P1: crate::Name = crate::Name::new_unchecked("vnd.ims.imsccv1p1");
    /// `vnd.ims.imsccv1p2`
    pub const IMS_IMSCCV1P2: crate::Name = crate::Name::new_unchecked("vnd.ims.imsccv1p2");
    /// `vnd.ims.imsccv1p3`
    pub const IMS_IMSCCV1P3: crate::Name = crate::Name::new_unchecked("vnd.ims.imsccv1p3");
    /// `vnd.ims.lis.v2.result`
    pub const IMS_LIS_V2_RESULT: crate::Name = crate::Name::new_unchecked("vnd.ims.lis.v2.result");
    /// `vnd.ims.lti.v2.toolconsumerprofile`
    pub const IMS_LTI_V2_TOOLCONSUMERPROFILE: crate::Name = crate::Name::new_unchecked("vnd.ims.lti.v2.toolconsumerprofile");
    /// `vnd.ims.lti.v2.toolproxy`
    pub const IMS_LTI_V2_TOOLPROXY: crate::Name = crate::Name::new_unchecked("vnd.ims.lti.v2.toolproxy");
    /// `vnd.ims.lti.v2.toolproxy.id`
    pub const IMS_LTI_V2_TOOLPROXY_ID: crate::Name = crate::Name::new_unchecked("vnd.ims.lti.v2.toolproxy.id");
    /// `vnd.ims.lti.v2.toolsettings`
    pub const IMS_LTI_V2_TOOLSETTINGS: crate::Name = crate::Name::new_unchecked("vnd.ims.lti.v2.toolsettings");
    /// `vnd.ims.lti.v2.toolsettings.simple`
    pub const IMS_LTI_V2_TOOLSETTINGS_SIMPLE: crate::Name = crate::Name::new_unchecked("vnd.ims.lti.v2.toolsettings.simple");
    /// `vnd.in3d.3dml`
    pub const IN3D_3DML: crate::Name = crate::Name::new_unchecked("vnd.in3d.3dml");
    /// `vnd.in3d.spot`
    pub const IN3D_SPOT: crate::Name = crate::Name::new_unchecked("vnd.in3d.spot");
    /// `vnd.informedcontrol.rms`
    pub const INFORMEDCONTROL_RMS: crate::Name = crate::Name::new_unchecked("vnd.informedcontrol.rms");
    /// `vnd.informix-visionary`
    pub const INFORMIX_VISIONARY: crate::Name = crate::Name::new_unchecked("vnd.informix-visionary");
    /// `vnd.infotech.project`
    pub const INFOTECH_PROJECT: crate::Name = crate::Name::new_unchecked("vnd.infotech.project");
    /// `vnd.innopath.wamp.notification`
    pub const INNOPATH_WAMP_NOTIFICATION: crate::Name = crate::Name::new_unchecked("vnd.innopath.wamp.notification");
    /// `vnd.insors.igm`
    pub const INSORS_IGM: crate::Name = crate::Name::new_unchecked("vnd.insors.igm");
    /// `vnd.intercon.formnet`
    pub const INTERCON_FORMNET: crate::Name = crate::Name::new_unchecked("vnd.intercon.formnet");
    /// `vnd.intergeo`
    pub const INTERGEO: crate::Name = crate::Name::new_unchecked("vnd.intergeo");
    /// `vnd.intertrust.digibox`
    pub const INTERTRUST_DIGIBOX: crate::Name = crate::Name::new_unchecked("vnd.intertrust.digibox");
    /// `vnd.intertrust.nncp`
    pub const INTERTRUST_NNCP: crate::Name = crate::Name::new_unchecked("vnd.intertrust.nncp");
    /// `vnd.intu.qbo`
    pub const INTU_QBO: crate::Name = crate::Name::new_unchecked("vnd.intu.qbo");
    /// `vnd.intu.qfx`
    pub const INTU_QFX: crate::Name = crate::Name::new_unchecked("vnd.intu.qfx");
    /// `vnd.iptc.g2.catalogitem`
    pub const IPTC_G2_CATALOGITEM: crate::Name = crate::Name::new_unchecked("vnd.iptc.g2.catalogitem");
    /// `vnd.iptc.g2.conceptitem`
    pub const IPTC_G2_CONCEPTITEM: crate::Name = crate::Name::new_unchecked("vnd.iptc.g2.conceptitem");
    /// `vnd.iptc.g2.knowledgeitem`
    pub const IPTC_G2_KNOWLEDGEITEM: crate::Name = crate::Name::new_unchecked("vnd.iptc.g2.knowledgeitem");
    /// `vnd.iptc.g2.newsitem`
    pub const IPTC_G2_NEWSITEM: crate::Name = crate::Name::new_unchecked("vnd.iptc.g2.newsitem");
    /// `vnd.iptc.g2.newsmessage`
    pub const IPTC_G2_NEWSMESSAGE: crate::Name = crate::Name::new_unchecked("vnd.iptc.g2.newsmessage");
    /// `vnd.iptc.g2.packageitem`
    pub const IPTC_G2_PACKAGEITEM: crate::Name = crate::Name::new_unchecked("vnd.iptc.g2.packageitem");
    /// `vnd.iptc.g2.planningitem`
    pub const IPTC_G2_PLANNINGITEM: crate::Name = crate::Name::new_unchecked("vnd.iptc.g2.planningitem");
    /// `vnd.IPTC.NewsML`
    pub const IPTC_NEWS_ML: crate::Name = crate::Name::new_unchecked("vnd.IPTC.NewsML");
    /// `vnd.IPTC.NITF`
    pub const IPTC_NITF: crate::Name = crate::Name::new_unchecked("vnd.IPTC.NITF");
    /// `vnd.iptvforum.1dparityfec-1010`
    pub const IPTVFORUM_1DPARITYFEC_1010: crate::Name = crate::Name::new_unchecked("vnd.iptvforum.1dparityfec-1010");
    /// `vnd.iptvforum.1dparityfec-2005`
    pub const IPTVFORUM_1DPARITYFEC_2005: crate::Name = crate::Name::new_unchecked("vnd.iptvforum.1dparityfec-2005");
    /// `vnd.iptvforum.2dparityfec-1010`
    pub const IPTVFORUM_2DPARITYFEC_1010: crate::Name = crate::Name::new_unchecked("vnd.iptvforum.2dparityfec-1010");
    /// `vnd.iptvforum.2dparityfec-2005`
    pub const IPTVFORUM_2DPARITYFEC_2005: crate::Name = crate::Name::new_unchecked("vnd.iptvforum.2dparityfec-2005");
    /// `vnd.iptvforum.ttsavc`
    pub const IPTVFORUM_TTSAVC: crate::Name = crate::Name::new_unchecked("vnd.iptvforum.ttsavc");
    /// `vnd.iptvforum.ttsmpeg2`
    pub const IPTVFORUM_TTSMPEG2: crate::Name = crate::Name::new_unchecked("vnd.iptvforum.ttsmpeg2");
    /// `vnd.ipunplugged.rcprofile`
    pub const IPUNPLUGGED_RCPROFILE: crate::Name = crate::Name::new_unchecked("vnd.ipunplugged.rcprofile");
    /// `vnd.irepository.package`
    pub const IREPOSITORY_PACKAGE: crate::Name = crate::Name::new_unchecked("vnd.irepository.package");
    /// `vnd.is-xpr`
    pub const IS_XPR: crate::Name = crate::Name::new_unchecked("vnd.is-xpr");
    /// `vnd.isac.fcs`
    pub const ISAC_FCS: crate::Name = crate::Name::new_unchecked("vnd.isac.fcs");
    /// `vnd.iso11783-10`
    pub const ISO11783_10: crate::Name = crate::Name::new_unchecked("vnd.iso11783-10");
    /// `vnd.jam`
    pub const JAM: crate::Name = crate::Name::new_unchecked("vnd.jam");
    /// `vnd.japannet-directory-service`
    pub const JAPANNET_DIRECTORY_SERVICE: crate::Name = crate::Name::new_unchecked("vnd.japannet-directory-service");
    /// `vnd.japannet-jpnstore-wakeup`
    pub const JAPANNET_JPNSTORE_WAKEUP: crate::Name = crate::Name::new_unchecked("vnd.japannet-jpnstore-wakeup");
    /// `vnd.japannet-payment-wakeup`
    pub const JAPANNET_PAYMENT_WAKEUP: crate::Name = crate::Name::new_unchecked("vnd.japannet-payment-wakeup");
    /// `vnd.japannet-registration`
    pub const JAPANNET_REGISTRATION: crate::Name = crate::Name::new_unchecked("vnd.japannet-registration");
    /// `vnd.japannet-registration-wakeup`
    pub const JAPANNET_REGISTRATION_WAKEUP: crate::Name = crate::Name::new_unchecked("vnd.japannet-registration-wakeup");
    /// `vnd.japannet-setstore-wakeup`
    pub const JAPANNET_SETSTORE_WAKEUP: crate::Name = crate::Name::new_unchecked("vnd.japannet-setstore-wakeup");
    /// `vnd.japannet-verification`
    pub const JAPANNET_VERIFICATION: crate::Name = crate::Name::new_unchecked("vnd.japannet-verification");
    /// `vnd.japannet-verification-wakeup`
    pub const JAPANNET_VERIFICATION_WAKEUP: crate::Name = crate::Name::new_unchecked("vnd.japannet-verification-wakeup");
    /// `vnd.jcp.javame.midlet-rms`
    pub const JCP_JAVAME_MIDLET_RMS: crate::Name = crate::Name::new_unchecked("vnd.jcp.javame.midlet-rms");
    /// `vnd.jisp`
    pub const JISP: crate::Name = crate::Name::new_unchecked("vnd.jisp");
    /// `vnd.joost.joda-archive`
    pub const JOOST_JODA_ARCHIVE: crate::Name = crate::Name::new_unchecked("vnd.joost.joda-archive");
    /// `vnd.jsk.isdn-ngn`
    pub const JSK_ISDN_NGN: crate::Name = crate::Name::new_unchecked("vnd.jsk.isdn-ngn");
    /// `vnd.kahootz`
    pub const KAHOOTZ: crate::Name = crate::Name::new_unchecked("vnd.kahootz");
    /// `vnd.kde.karbon`
    pub const KDE_KARBON: crate::Name = crate::Name::new_unchecked("vnd.kde.karbon");
    /// `vnd.kde.kchart`
    pub const KDE_KCHART: crate::Name = crate::Name::new_unchecked("vnd.kde.kchart");
    /// `vnd.kde.kformula`
    pub const KDE_KFORMULA: crate::Name = crate::Name::new_unchecked("vnd.kde.kformula");
    /// `vnd.kde.kivio`
    pub const KDE_KIVIO: crate::Name = crate::Name::new_unchecked("vnd.kde.kivio");
    /// `vnd.kde.kontour`
    pub const KDE_KONTOUR: crate::Name = crate::Name::new_unchecked("vnd.kde.kontour");
    /// `vnd.kde.kpresenter`
    pub const KDE_KPRESENTER: crate::Name = crate::Name::new_unchecked("vnd.kde.kpresenter");
    /// `vnd.kde.kspread`
    pub const KDE_KSPREAD: crate::Name = crate::Name::new_unchecked("vnd.kde.kspread");
    /// `vnd.kde.kword`
    pub const KDE_KWORD: crate::Name = crate::Name::new_unchecked("vnd.kde.kword");
    /// `vnd.kenameaapp`
    pub const KENAMEAAPP: crate::Name = crate::Name::new_unchecked("vnd.kenameaapp");
    /// `vnd.kidspiration`
    pub const KIDSPIRATION: crate::Name = crate::Name::new_unchecked("vnd.kidspiration");
    /// `vnd.Kinar`
    pub const KINAR: crate::Name = crate::Name::new_unchecked("vnd.Kinar");
    /// `vnd.koan`
    pub const KOAN: crate::Name = crate::Name::new_unchecked("vnd.koan");
    /// `vnd.kodak-descriptor`
    pub const KODAK_DESCRIPTOR: crate::Name = crate::Name::new_unchecked("vnd.kodak-descriptor");
    /// `vnd.las`
    pub const LAS: crate::Name = crate::Name::new_unchecked("vnd.las");
    /// `vnd.las.las`
    pub const LAS_LAS: crate::Name = crate::Name::new_unchecked("vnd.las.las");
    /// `vnd.laszip`
    pub const LASZIP: crate::Name = crate::Name::new_unchecked("vnd.laszip");
    /// `vnd.latex-z`
    pub const LATEX_Z: crate::Name = crate::Name::new_unchecked("vnd.latex-z");
    /// `vnd.leap`
    pub const LEAP: crate::Name = crate::Name::new_unchecked("vnd.leap");
    /// `vnd.liberty-request`
    pub const LIBERTY_REQUEST: crate::Name = crate::Name::new_unchecked("vnd.liberty-request");
    /// `vnd.llamagraphics.life-balance.desktop`
    pub const LLAMAGRAPHICS_LIFE_BALANCE_DESKTOP: crate::Name = crate::Name::new_unchecked("vnd.llamagraphics.life-balance.desktop");
    /// `vnd.llamagraphics.life-balance.exchange`
    pub const LLAMAGRAPHICS_LIFE_BALANCE_EXCHANGE: crate::Name = crate::Name::new_unchecked("vnd.llamagraphics.life-balance.exchange");
    /// `vnd.logipipe.circuit`
    pub const LOGIPIPE_CIRCUIT: crate::Name = crate::Name::new_unchecked("vnd.logipipe.circuit");
    /// `vnd.loom`
    pub const LOOM: crate::Name = crate::Name::new_unchecked("vnd.loom");
    /// `vnd.lotus-1-2-3`
    pub const LOTUS_1_2_3: crate::Name = crate::Name::new_unchecked("vnd.lotus-1-2-3");
    /// `vnd.lotus-approach`
    pub const LOTUS_APPROACH: crate::Name = crate::Name::new_unchecked("vnd.lotus-approach");
    /// `vnd.lotus-freelance`
    pub const LOTUS_FREELANCE: crate::Name = crate::Name::new_unchecked("vnd.lotus-freelance");
    /// `vnd.lotus-notes`
    pub const LOTUS_NOTES: crate::Name = crate::Name::new_unchecked("vnd.lotus-notes");
    /// `vnd.lotus-organizer`
    pub const LOTUS_ORGANIZER: crate::Name = crate::Name::new_unchecked("vnd.lotus-organizer");
    /// `vnd.lotus-screencam`
    pub const LOTUS_SCREENCAM: crate::Name = crate::Name::new_unchecked("vnd.lotus-screencam");
    /// `vnd.lotus-wordpro`
    pub const LOTUS_WORDPRO: crate::Name = crate::Name::new_unchecked("vnd.lotus-wordpro");
    /// `vnd.lucent.voice`
    pub const LUCENT_VOICE: crate::Name = crate::Name::new_unchecked("vnd.lucent.voice");
    /// `vnd.macports.portpkg`
    pub const MACPORTS_PORTPKG: crate::Name = crate::Name::new_unchecked("vnd.macports.portpkg");
    /// `vnd.mapbox-vector-tile`
    pub const MAPBOX_VECTOR_TILE: crate::Name = crate::Name::new_unchecked("vnd.mapbox-vector-tile");
    /// `vnd.marlin.drm.actiontoken`
    pub const MARLIN_DRM_ACTIONTOKEN: crate::Name = crate::Name::new_unchecked("vnd.marlin.drm.actiontoken");
    /// `vnd.marlin.drm.conftoken`
    pub const MARLIN_DRM_CONFTOKEN: crate::Name = crate::Name::new_unchecked("vnd.marlin.drm.conftoken");
    /// `vnd.marlin.drm.license`
    pub const MARLIN_DRM_LICENSE: crate::Name = crate::Name::new_unchecked("vnd.marlin.drm.license");
    /// `vnd.marlin.drm.mdcf`
    pub const MARLIN_DRM_MDCF: crate::Name = crate::Name::new_unchecked("vnd.marlin.drm.mdcf");
    /// `vnd.mason`
    pub const MASON: crate::Name = crate::Name::new_unchecked("vnd.mason");
    /// `vnd.maxar.archive.3tz`
    pub const MAXAR_ARCHIVE_3TZ: crate::Name = crate::Name::new_unchecked("vnd.maxar.archive.3tz");
    /// `vnd.maxmind.maxmind-db`
    pub const MAXMIND_MAXMIND_DB: crate::Name = crate::Name::new_unchecked("vnd.maxmind.maxmind-db");
    /// `vnd.mcd`
    pub const MCD: crate::Name = crate::Name::new_unchecked("vnd.mcd");
    /// `vnd.medcalcdata`
    pub const MEDCALCDATA: crate::Name = crate::Name::new_unchecked("vnd.medcalcdata");
    /// `vnd.mediastation.cdkey`
    pub const MEDIASTATION_CDKEY: crate::Name = crate::Name::new_unchecked("vnd.mediastation.cdkey");
    /// `vnd.meridian-slingshot`
    pub const MERIDIAN_SLINGSHOT: crate::Name = crate::Name::new_unchecked("vnd.meridian-slingshot");
    /// `vnd.MFER`
    pub const MFER: crate::Name = crate::Name::new_unchecked("vnd.MFER");
    /// `vnd.mfmp`
    pub const MFMP: crate::Name = crate::Name::new_unchecked("vnd.mfmp");
    /// `vnd.micro`
    pub const MICRO: crate::Name = crate::Name::new_unchecked("vnd.micro");
    /// `vnd.micrografx.flo`
    pub const MICROGRAFX_FLO: crate::Name = crate::Name::new_unchecked("vnd.micrografx.flo");
    /// `vnd.micrografx.igx`
    pub const MICROGRAFX_IGX: crate::Name = crate::Name::new_unchecked("vnd.micrografx.igx");
    /// `vnd.microsoft.icon`
    pub const MICROSOFT_ICON: crate::Name = crate::Name::new_unchecked("vnd.microsoft.icon");
    /// `vnd.microsoft.portable-executable`
    pub const MICROSOFT_PORTABLE_EXECUTABLE: crate::Name = crate::Name::new_unchecked("vnd.microsoft.portable-executable");
    /// `vnd.microsoft.windows.thumbnail-cache`
    pub const MICROSOFT_WINDOWS_THUMBNAIL_CACHE: crate::Name = crate::Name::new_unchecked("vnd.microsoft.windows.thumbnail-cache");
    /// `vnd.miele`
    pub const MIELE: crate::Name = crate::Name::new_unchecked("vnd.miele");
    /// `vnd.mif`
    pub const MIF: crate::Name = crate::Name::new_unchecked("vnd.mif");
    /// `vnd.minisoft-hp3000-save`
    pub const MINISOFT_HP3000_SAVE: crate::Name = crate::Name::new_unchecked("vnd.minisoft-hp3000-save");
    /// `vnd.mitsubishi.misty-guard.trustweb`
    pub const MITSUBISHI_MISTY_GUARD_TRUSTWEB: crate::Name = crate::Name::new_unchecked("vnd.mitsubishi.misty-guard.trustweb");
    /// `vnd.mix`
    pub const MIX: crate::Name = crate::Name::new_unchecked("vnd.mix");
    /// `vnd.Mobius.DAF`
    pub const MOBIUS_DAF: crate::Name = crate::Name::new_unchecked("vnd.Mobius.DAF");
    /// `vnd.Mobius.DIS`
    pub const MOBIUS_DIS: crate::Name = crate::Name::new_unchecked("vnd.Mobius.DIS");
    /// `vnd.Mobius.MBK`
    pub const MOBIUS_MBK: crate::Name = crate::Name::new_unchecked("vnd.Mobius.MBK");
    /// `vnd.Mobius.MQY`
    pub const MOBIUS_MQY: crate::Name = crate::Name::new_unchecked("vnd.Mobius.MQY");
    /// `vnd.Mobius.MSL`
    pub const MOBIUS_MSL: crate::Name = crate::Name::new_unchecked("vnd.Mobius.MSL");
    /// `vnd.Mobius.PLC`
    pub const MOBIUS_PLC: crate::Name = crate::Name::new_unchecked("vnd.Mobius.PLC");
    /// `vnd.Mobius.TXF`
    pub const MOBIUS_TXF: crate::Name = crate::Name::new_unchecked("vnd.Mobius.TXF");
    /// `vnd.moml`
    pub const MOML: crate::Name = crate::Name::new_unchecked("vnd.moml");
    /// `vnd.mophun.application`
    pub const MOPHUN_APPLICATION: crate::Name = crate::Name::new_unchecked("vnd.mophun.application");
    /// `vnd.mophun.certificate`
    pub const MOPHUN_CERTIFICATE: crate::Name = crate::Name::new_unchecked("vnd.mophun.certificate");
    /// `vnd.motorola.flexsuite`
    pub const MOTOROLA_FLEXSUITE: crate::Name = crate::Name::new_unchecked("vnd.motorola.flexsuite");
    /// `vnd.motorola.flexsuite.adsi`
    pub const MOTOROLA_FLEXSUITE_ADSI: crate::Name = crate::Name::new_unchecked("vnd.motorola.flexsuite.adsi");
    /// `vnd.motorola.flexsuite.fis`
    pub const MOTOROLA_FLEXSUITE_FIS: crate::Name = crate::Name::new_unchecked("vnd.motorola.flexsuite.fis");
    /// `vnd.motorola.flexsuite.gotap`
    pub const MOTOROLA_FLEXSUITE_GOTAP: crate::Name = crate::Name::new_unchecked("vnd.motorola.flexsuite.gotap");
    /// `vnd.motorola.flexsuite.kmr`
    pub const MOTOROLA_FLEXSUITE_KMR: crate::Name = crate::Name::new_unchecked("vnd.motorola.flexsuite.kmr");
    /// `vnd.motorola.flexsuite.ttc`
    pub const MOTOROLA_FLEXSUITE_TTC: crate::Name = crate::Name::new_unchecked("vnd.motorola.flexsuite.ttc");
    /// `vnd.motorola.flexsuite.wem`
    pub const MOTOROLA_FLEXSUITE_WEM: crate::Name = crate::Name::new_unchecked("vnd.motorola.flexsuite.wem");
    /// `vnd.motorola.iprm`
    pub const MOTOROLA_IPRM: crate::Name = crate::Name::new_unchecked("vnd.motorola.iprm");
    /// `vnd.motorola.reflex`
    pub const MOTOROLA_REFLEX: crate::Name = crate::Name::new_unchecked("vnd.motorola.reflex");
    /// `vnd.motorola.video`
    pub const MOTOROLA_VIDEO: crate::Name = crate::Name::new_unchecked("vnd.motorola.video");
    /// `vnd.motorola.videop`
    pub const MOTOROLA_VIDEOP: crate::Name = crate::Name::new_unchecked("vnd.motorola.videop");
    /// `vnd.mozilla.apng`
    pub const MOZILLA_APNG: crate::Name = crate::Name::new_unchecked("vnd.mozilla.apng");
    /// `vnd.mozilla.xul`
    pub const MOZILLA_XUL: crate::Name = crate::Name::new_unchecked("vnd.mozilla.xul");
    /// `vnd.mpegurl`
    pub const MPEGURL: crate::Name = crate::Name::new_unchecked("vnd.mpegurl");
    /// `vnd.ms-3mfdocument`
    pub const MS_3MFDOCUMENT: crate::Name = crate::Name::new_unchecked("vnd.ms-3mfdocument");
    /// `vnd.ms-artgalry`
    pub const MS_ARTGALRY: crate::Name = crate::Name::new_unchecked("vnd.ms-artgalry");
    /// `vnd.ms-asf`
    pub const MS_ASF: crate::Name = crate::Name::new_unchecked("vnd.ms-asf");
    /// `vnd.ms-cab-compressed`
    pub const MS_CAB_COMPRESSED: crate::Name = crate::Name::new_unchecked("vnd.ms-cab-compressed");
    /// `vnd.ms-excel`
    pub const MS_EXCEL: crate::Name = crate::Name::new_unchecked("vnd.ms-excel");
    /// `vnd.ms-excel.addin.macroEnabled.12`
    pub const MS_EXCEL_ADDIN_MACRO_ENABLED_12: crate::Name = crate::Name::new_unchecked("vnd.ms-excel.addin.macroEnabled.12");
    /// `vnd.ms-excel.sheet.binary.macroEnabled.12`
    pub const MS_EXCEL_SHEET_BINARY_MACRO_ENABLED_12: crate::Name = crate::Name::new_unchecked("vnd.ms-excel.sheet.binary.macroEnabled.12");
    /// `vnd.ms-excel.sheet.macroEnabled.12`
    pub const MS_EXCEL_SHEET_MACRO_ENABLED_12: crate::Name = crate::Name::new_unchecked("vnd.ms-excel.sheet.macroEnabled.12");
    /// `vnd.ms-excel.template.macroEnabled.12`
    pub const MS_EXCEL_TEMPLATE_MACRO_ENABLED_12: crate::Name = crate::Name::new_unchecked("vnd.ms-excel.template.macroEnabled.12");
    /// `vnd.ms-fontobject`
    pub const MS_FONTOBJECT: crate::Name = crate::Name::new_unchecked("vnd.ms-fontobject");
    /// `vnd.ms-htmlhelp`
    pub const MS_HTMLHELP: crate::Name = crate::Name::new_unchecked("vnd.ms-htmlhelp");
    /// `vnd.ms-ims`
    pub const MS_IMS: crate::Name = crate::Name::new_unchecked("vnd.ms-ims");
    /// `vnd.ms-lrm`
    pub const MS_LRM: crate::Name = crate::Name::new_unchecked("vnd.ms-lrm");
    /// `vnd.ms-mediapackage`
    pub const MS_MEDIAPACKAGE: crate::Name = crate::Name::new_unchecked("vnd.ms-mediapackage");
    /// `vnd.ms-modi`
    pub const MS_MODI: crate::Name = crate::Name::new_unchecked("vnd.ms-modi");
    /// `vnd.ms-office.activeX`
    pub const MS_OFFICE_ACTIVE_X: crate::Name = crate::Name::new_unchecked("vnd.ms-office.activeX");
    /// `vnd.ms-officetheme`
    pub const MS_OFFICETHEME: crate::Name = crate::Name::new_unchecked("vnd.ms-officetheme");
    /// `vnd.ms-playready.initiator`
    pub const MS_PLAYREADY_INITIATOR: crate::Name = crate::Name::new_unchecked("vnd.ms-playready.initiator");
    /// `vnd.ms-playready.media.pya`
    pub const MS_PLAYREADY_MEDIA_PYA: crate::Name = crate::Name::new_unchecked("vnd.ms-playready.media.pya");
    /// `vnd.ms-playready.media.pyv`
    pub const MS_PLAYREADY_MEDIA_PYV: crate::Name = crate::Name::new_unchecked("vnd.ms-playready.media.pyv");
    /// `vnd.ms-powerpoint`
    pub const MS_POWERPOINT: crate::Name = crate::Name::new_unchecked("vnd.ms-powerpoint");
    /// `vnd.ms-powerpoint.addin.macroEnabled.12`
    pub const MS_POWERPOINT_ADDIN_MACRO_ENABLED_12: crate::Name = crate::Name::new_unchecked("vnd.ms-powerpoint.addin.macroEnabled.12");
    /// `vnd.ms-powerpoint.presentation.macroEnabled.12`
    pub const MS_POWERPOINT_PRESENTATION_MACRO_ENABLED_12: crate::Name = crate::Name::new_unchecked("vnd.ms-powerpoint.presentation.macroEnabled.12");
    /// `vnd.ms-powerpoint.slide.macroEnabled.12`
    pub const MS_POWERPOINT_SLIDE_MACRO_ENABLED_12: crate::Name = crate::Name::new_unchecked("vnd.ms-powerpoint.slide.macroEnabled.12");
    /// `vnd.ms-powerpoint.slideshow.macroEnabled.12`
    pub const MS_POWERPOINT_SLIDESHOW_MACRO_ENABLED_12: crate::Name = crate::Name::new_unchecked("vnd.ms-powerpoint.slideshow.macroEnabled.12");
    /// `vnd.ms-powerpoint.template.macroEnabled.12`
    pub const MS_POWERPOINT_TEMPLATE_MACRO_ENABLED_12: crate::Name = crate::Name::new_unchecked("vnd.ms-powerpoint.template.macroEnabled.12");
    /// `vnd.ms-PrintDeviceCapabilities`
    pub const MS_PRINT_DEVICE_CAPABILITIES: crate::Name = crate::Name::new_unchecked("vnd.ms-PrintDeviceCapabilities");
    /// `vnd.ms-PrintSchemaTicket`
    pub const MS_PRINT_SCHEMA_TICKET: crate::Name = crate::Name::new_unchecked("vnd.ms-PrintSchemaTicket");
    /// `vnd.ms-project`
    pub const MS_PROJECT: crate::Name = crate::Name::new_unchecked("vnd.ms-project");
    /// `vnd.ms-tnef`
    pub const MS_TNEF: crate::Name = crate::Name::new_unchecked("vnd.ms-tnef");
    /// `vnd.ms-windows.devicepairing`
    pub const MS_WINDOWS_DEVICEPAIRING: crate::Name = crate::Name::new_unchecked("vnd.ms-windows.devicepairing");
    /// `vnd.ms-windows.nwprinting.oob`
    pub const MS_WINDOWS_NWPRINTING_OOB: crate::Name = crate::Name::new_unchecked("vnd.ms-windows.nwprinting.oob");
    /// `vnd.ms-windows.printerpairing`
    pub const MS_WINDOWS_PRINTERPAIRING: crate::Name = crate::Name::new_unchecked("vnd.ms-windows.printerpairing");
    /// `vnd.ms-windows.wsd.oob`
    pub const MS_WINDOWS_WSD_OOB: crate::Name = crate::Name::new_unchecked("vnd.ms-windows.wsd.oob");
    /// `vnd.ms-wmdrm.lic-chlg-req`
    pub const MS_WMDRM_LIC_CHLG_REQ: crate::Name = crate::Name::new_unchecked("vnd.ms-wmdrm.lic-chlg-req");
    /// `vnd.ms-wmdrm.lic-resp`
    pub const MS_WMDRM_LIC_RESP: crate::Name = crate::Name::new_unchecked("vnd.ms-wmdrm.lic-resp");
    /// `vnd.ms-wmdrm.meter-chlg-req`
    pub const MS_WMDRM_METER_CHLG_REQ: crate::Name = crate::Name::new_unchecked("vnd.ms-wmdrm.meter-chlg-req");
    /// `vnd.ms-wmdrm.meter-resp`
    pub const MS_WMDRM_METER_RESP: crate::Name = crate::Name::new_unchecked("vnd.ms-wmdrm.meter-resp");
    /// `vnd.ms-word.document.macroEnabled.12`
    pub const MS_WORD_DOCUMENT_MACRO_ENABLED_12: crate::Name = crate::Name::new_unchecked("vnd.ms-word.document.macroEnabled.12");
    /// `vnd.ms-word.template.macroEnabled.12`
    pub const MS_WORD_TEMPLATE_MACRO_ENABLED_12: crate::Name = crate::Name::new_unchecked("vnd.ms-word.template.macroEnabled.12");
    /// `vnd.ms-works`
    pub const MS_WORKS: crate::Name = crate::Name::new_unchecked("vnd.ms-works");
    /// `vnd.ms-wpl`
    pub const MS_WPL: crate::Name = crate::Name::new_unchecked("vnd.ms-wpl");
    /// `vnd.ms-xpsdocument`
    pub const MS_XPSDOCUMENT: crate::Name = crate::Name::new_unchecked("vnd.ms-xpsdocument");
    /// `vnd.msa-disk-image`
    pub const MSA_DISK_IMAGE: crate::Name = crate::Name::new_unchecked("vnd.msa-disk-image");
    /// `vnd.mseq`
    pub const MSEQ: crate::Name = crate::Name::new_unchecked("vnd.mseq");
    /// `vnd.msign`
    pub const MSIGN: crate::Name = crate::Name::new_unchecked("vnd.msign");
    /// `vnd.mts`
    pub const MTS: crate::Name = crate::Name::new_unchecked("vnd.mts");
    /// `vnd.multiad.creator`
    pub const MULTIAD_CREATOR: crate::Name = crate::Name::new_unchecked("vnd.multiad.creator");
    /// `vnd.multiad.creator.cif`
    pub const MULTIAD_CREATOR_CIF: crate::Name = crate::Name::new_unchecked("vnd.multiad.creator.cif");
    /// `vnd.music-niff`
    pub const MUSIC_NIFF: crate::Name = crate::Name::new_unchecked("vnd.music-niff");
    /// `vnd.musician`
    pub const MUSICIAN: crate::Name = crate::Name::new_unchecked("vnd.musician");
    /// `vnd.muvee.style`
    pub const MUVEE_STYLE: crate::Name = crate::Name::new_unchecked("vnd.muvee.style");
    /// `vnd.mynfc`
    pub const MYNFC: crate::Name = crate::Name::new_unchecked("vnd.mynfc");
    /// `vnd.nacamar.ybrid`
    pub const NACAMAR_YBRID: crate::Name = crate::Name::new_unchecked("vnd.nacamar.ybrid");
    /// `vnd.ncd.control`
    pub const NCD_CONTROL: crate::Name = crate::Name::new_unchecked("vnd.ncd.control");
    /// `vnd.ncd.reference`
    pub const NCD_REFERENCE: crate::Name = crate::Name::new_unchecked("vnd.ncd.reference");
    /// `vnd.nearst.inv`
    pub const NEARST_INV: crate::Name = crate::Name::new_unchecked("vnd.nearst.inv");
    /// `vnd.nebumind.line`
    pub const NEBUMIND_LINE: crate::Name = crate::Name::new_unchecked("vnd.nebumind.line");
    /// `vnd.nervana`
    pub const NERVANA: crate::Name = crate::Name::new_unchecked("vnd.nervana");
    /// `vnd.net-fpx`
    pub const NET_FPX: crate::Name = crate::Name::new_unchecked("vnd.net-fpx");
    /// `vnd.net2phone.commcenter.command`
    pub const NET2PHONE_COMMCENTER_COMMAND: crate::Name = crate::Name::new_unchecked("vnd.net2phone.commcenter.command");
    /// `vnd.netfpx`
    pub const NETFPX: crate::Name = crate::Name::new_unchecked("vnd.netfpx");
    /// `vnd.neurolanguage.nlu`
    pub const NEUROLANGUAGE_NLU: crate::Name = crate::Name::new_unchecked("vnd.neurolanguage.nlu");
    /// `vnd.nimn`
    pub const NIMN: crate::Name = crate::Name::new_unchecked("vnd.nimn");
    /// `vnd.nintendo.nitro.rom`
    pub const NINTENDO_NITRO_ROM: crate::Name = crate::Name::new_unchecked("vnd.nintendo.nitro.rom");
    /// `vnd.nintendo.snes.rom`
    pub const NINTENDO_SNES_ROM: crate::Name = crate::Name::new_unchecked("vnd.nintendo.snes.rom");
    /// `vnd.nitf`
    pub const NITF: crate::Name = crate::Name::new_unchecked("vnd.nitf");
    /// `vnd.noblenet-directory`
    pub const NOBLENET_DIRECTORY: crate::Name = crate::Name::new_unchecked("vnd.noblenet-directory");
    /// `vnd.noblenet-sealer`
    pub const NOBLENET_SEALER: crate::Name = crate::Name::new_unchecked("vnd.noblenet-sealer");
    /// `vnd.noblenet-web`
    pub const NOBLENET_WEB: crate::Name = crate::Name::new_unchecked("vnd.noblenet-web");
    /// `vnd.nokia.catalogs`
    pub const NOKIA_CATALOGS: crate::Name = crate::Name::new_unchecked("vnd.nokia.catalogs");
    /// `vnd.nokia.conml`
    pub const NOKIA_CONML: crate::Name = crate::Name::new_unchecked("vnd.nokia.conml");
    /// `vnd.nokia.interleaved-multimedia`
    pub const NOKIA_INTERLEAVED_MULTIMEDIA: crate::Name = crate::Name::new_unchecked("vnd.nokia.interleaved-multimedia");
    /// `vnd.nokia.iptv.config`
    pub const NOKIA_IPTV_CONFIG: crate::Name = crate::Name::new_unchecked("vnd.nokia.iptv.config");
    /// `vnd.nokia.iSDS-radio-presets`
    pub const NOKIA_I_SDS_RADIO_PRESETS: crate::Name = crate::Name::new_unchecked("vnd.nokia.iSDS-radio-presets");
    /// `vnd.nokia.landmark`
    pub const NOKIA_LANDMARK: crate::Name = crate::Name::new_unchecked("vnd.nokia.landmark");
    /// `vnd.nokia.landmarkcollection`
    pub const NOKIA_LANDMARKCOLLECTION: crate::Name = crate::Name::new_unchecked("vnd.nokia.landmarkcollection");
    /// `vnd.nokia.mobile-xmf`
    pub const NOKIA_MOBILE_XMF: crate::Name = crate::Name::new_unchecked("vnd.nokia.mobile-xmf");
    /// `vnd.nokia.mp4vr`
    pub const NOKIA_MP4VR: crate::Name = crate::Name::new_unchecked("vnd.nokia.mp4vr");
    /// `vnd.nokia.n-gage.ac`
    pub const NOKIA_N_GAGE_AC: crate::Name = crate::Name::new_unchecked("vnd.nokia.n-gage.ac");
    /// `vnd.nokia.n-gage.data`
    pub const NOKIA_N_GAGE_DATA: crate::Name = crate::Name::new_unchecked("vnd.nokia.n-gage.data");
    /// `vnd.nokia.n-gage.symbian.install`
    pub const NOKIA_N_GAGE_SYMBIAN_INSTALL: crate::Name = crate::Name::new_unchecked("vnd.nokia.n-gage.symbian.install");
    /// `vnd.nokia.ncd`
    pub const NOKIA_NCD: crate::Name = crate::Name::new_unchecked("vnd.nokia.ncd");
    /// `vnd.nokia.pcd`
    pub const NOKIA_PCD: crate::Name = crate::Name::new_unchecked("vnd.nokia.pcd");
    /// `vnd.nokia.radio-preset`
    pub const NOKIA_RADIO_PRESET: crate::Name = crate::Name::new_unchecked("vnd.nokia.radio-preset");
    /// `vnd.nokia.radio-presets`
    pub const NOKIA_RADIO_PRESETS: crate::Name = crate::Name::new_unchecked("vnd.nokia.radio-presets");
    /// `vnd.nokia.videovoip`
    pub const NOKIA_VIDEOVOIP: crate::Name = crate::Name::new_unchecked("vnd.nokia.videovoip");
    /// `vnd.nortel.vbk`
    pub const NORTEL_VBK: crate::Name = crate::Name::new_unchecked("vnd.nortel.vbk");
    /// `vnd.novadigm.EDM`
    pub const NOVADIGM_EDM: crate::Name = crate::Name::new_unchecked("vnd.novadigm.EDM");
    /// `vnd.novadigm.EDX`
    pub const NOVADIGM_EDX: crate::Name = crate::Name::new_unchecked("vnd.novadigm.EDX");
    /// `vnd.novadigm.EXT`
    pub const NOVADIGM_EXT: crate::Name = crate::Name::new_unchecked("vnd.novadigm.EXT");
    /// `vnd.ntt-local.content-share`
    pub const NTT_LOCAL_CONTENT_SHARE: crate::Name = crate::Name::new_unchecked("vnd.ntt-local.content-share");
    /// `vnd.ntt-local.file-transfer`
    pub const NTT_LOCAL_FILE_TRANSFER: crate::Name = crate::Name::new_unchecked("vnd.ntt-local.file-transfer");
    /// `vnd.ntt-local.ogw_remote-access`
    pub const NTT_LOCAL_OGW_REMOTE_ACCESS: crate::Name = crate::Name::new_unchecked("vnd.ntt-local.ogw_remote-access");
    /// `vnd.ntt-local.sip-ta_remote`
    pub const NTT_LOCAL_SIP_TA_REMOTE: crate::Name = crate::Name::new_unchecked("vnd.ntt-local.sip-ta_remote");
    /// `vnd.ntt-local.sip-ta_tcp_stream`
    pub const NTT_LOCAL_SIP_TA_TCP_STREAM: crate::Name = crate::Name::new_unchecked("vnd.ntt-local.sip-ta_tcp_stream");
    /// `vnd.nuera.ecelp4800`
    pub const NUERA_ECELP4800: crate::Name = crate::Name::new_unchecked("vnd.nuera.ecelp4800");
    /// `vnd.nuera.ecelp7470`
    pub const NUERA_ECELP7470: crate::Name = crate::Name::new_unchecked("vnd.nuera.ecelp7470");
    /// `vnd.nuera.ecelp9600`
    pub const NUERA_ECELP9600: crate::Name = crate::Name::new_unchecked("vnd.nuera.ecelp9600");
    /// `vnd.oasis.opendocument.chart`
    pub const OASIS_OPENDOCUMENT_CHART: crate::Name = crate::Name::new_unchecked("vnd.oasis.opendocument.chart");
    /// `vnd.oasis.opendocument.chart-template`
    pub const OASIS_OPENDOCUMENT_CHART_TEMPLATE: crate::Name = crate::Name::new_unchecked("vnd.oasis.opendocument.chart-template");
    /// `vnd.oasis.opendocument.database`
    pub const OASIS_OPENDOCUMENT_DATABASE: crate::Name = crate::Name::new_unchecked("vnd.oasis.opendocument.database");
    /// `vnd.oasis.opendocument.formula`
    pub const OASIS_OPENDOCUMENT_FORMULA: crate::Name = crate::Name::new_unchecked("vnd.oasis.opendocument.formula");
    /// `vnd.oasis.opendocument.formula-template`
    pub const OASIS_OPENDOCUMENT_FORMULA_TEMPLATE: crate::Name = crate::Name::new_unchecked("vnd.oasis.opendocument.formula-template");
    /// `vnd.oasis.opendocument.graphics`
    pub const OASIS_OPENDOCUMENT_GRAPHICS: crate::Name = crate::Name::new_unchecked("vnd.oasis.opendocument.graphics");
    /// `vnd.oasis.opendocument.graphics-template`
    pub const OASIS_OPENDOCUMENT_GRAPHICS_TEMPLATE: crate::Name = crate::Name::new_unchecked("vnd.oasis.opendocument.graphics-template");
    /// `vnd.oasis.opendocument.image`
    pub const OASIS_OPENDOCUMENT_IMAGE: crate::Name = crate::Name::new_unchecked("vnd.oasis.opendocument.image");
    /// `vnd.oasis.opendocument.image-template`
    pub const OASIS_OPENDOCUMENT_IMAGE_TEMPLATE: crate::Name = crate::Name::new_unchecked("vnd.oasis.opendocument.image-template");
    /// `vnd.oasis.opendocument.presentation`
    pub const OASIS_OPENDOCUMENT_PRESENTATION: crate::Name = crate::Name::new_unchecked("vnd.oasis.opendocument.presentation");
    /// `vnd.oasis.opendocument.presentation-template`
    pub const OASIS_OPENDOCUMENT_PRESENTATION_TEMPLATE: crate::Name = crate::Name::new_unchecked("vnd.oasis.opendocument.presentation-template");
    /// `vnd.oasis.opendocument.spreadsheet`
    pub const OASIS_OPENDOCUMENT_SPREADSHEET: crate::Name = crate::Name::new_unchecked("vnd.oasis.opendocument.spreadsheet");
    /// `vnd.oasis.opendocument.spreadsheet-template`
    pub const OASIS_OPENDOCUMENT_SPREADSHEET_TEMPLATE: crate::Name = crate::Name::new_unchecked("vnd.oasis.opendocument.spreadsheet-template");
    /// `vnd.oasis.opendocument.text`
    pub const OASIS_OPENDOCUMENT_TEXT: crate::Name = crate::Name::new_unchecked("vnd.oasis.opendocument.text");
    /// `vnd.oasis.opendocument.text-master`
    pub const OASIS_OPENDOCUMENT_TEXT_MASTER: crate::Name = crate::Name::new_unchecked("vnd.oasis.opendocument.text-master");
    /// `vnd.oasis.opendocument.text-template`
    pub const OASIS_OPENDOCUMENT_TEXT_TEMPLATE: crate::Name = crate::Name::new_unchecked("vnd.oasis.opendocument.text-template");
    /// `vnd.oasis.opendocument.text-web`
    pub const OASIS_OPENDOCUMENT_TEXT_WEB: crate::Name = crate::Name::new_unchecked("vnd.oasis.opendocument.text-web");
    /// `vnd.objectvideo`
    pub const OBJECTVIDEO: crate::Name = crate::Name::new_unchecked("vnd.objectvideo");
    /// `vnd.obn`
    pub const OBN: crate::Name = crate::Name::new_unchecked("vnd.obn");
    /// `vnd.ocf`
    pub const OCF: crate::Name = crate::Name::new_unchecked("vnd.ocf");
    /// `vnd.oci.image.manifest.v1`
    pub const OCI_IMAGE_MANIFEST_V1: crate::Name = crate::Name::new_unchecked("vnd.oci.image.manifest.v1");
    /// `vnd.octel.sbc`
    pub const OCTEL_SBC: crate::Name = crate::Name::new_unchecked("vnd.octel.sbc");
    /// `vnd.oftn.l10n`
    pub const OFTN_L10N: crate::Name = crate::Name::new_unchecked("vnd.oftn.l10n");
    /// `vnd.oipf.contentaccessdownload`
    pub const OIPF_CONTENTACCESSDOWNLOAD: crate::Name = crate::Name::new_unchecked("vnd.oipf.contentaccessdownload");
    /// `vnd.oipf.contentaccessstreaming`
    pub const OIPF_CONTENTACCESSSTREAMING: crate::Name = crate::Name::new_unchecked("vnd.oipf.contentaccessstreaming");
    /// `vnd.oipf.cspg-hexbinary`
    pub const OIPF_CSPG_HEXBINARY: crate::Name = crate::Name::new_unchecked("vnd.oipf.cspg-hexbinary");
    /// `vnd.oipf.dae.svg`
    pub const OIPF_DAE_SVG: crate::Name = crate::Name::new_unchecked("vnd.oipf.dae.svg");
    /// `vnd.oipf.dae.xhtml`
    pub const OIPF_DAE_XHTML: crate::Name = crate::Name::new_unchecked("vnd.oipf.dae.xhtml");
    /// `vnd.oipf.mippvcontrolmessage`
    pub const OIPF_MIPPVCONTROLMESSAGE: crate::Name = crate::Name::new_unchecked("vnd.oipf.mippvcontrolmessage");
    /// `vnd.oipf.pae.gem`
    pub const OIPF_PAE_GEM: crate::Name = crate::Name::new_unchecked("vnd.oipf.pae.gem");
    /// `vnd.oipf.spdiscovery`
    pub const OIPF_SPDISCOVERY: crate::Name = crate::Name::new_unchecked("vnd.oipf.spdiscovery");
    /// `vnd.oipf.spdlist`
    pub const OIPF_SPDLIST: crate::Name = crate::Name::new_unchecked("vnd.oipf.spdlist");
    /// `vnd.oipf.ueprofile`
    pub const OIPF_UEPROFILE: crate::Name = crate::Name::new_unchecked("vnd.oipf.ueprofile");
    /// `vnd.oipf.userprofile`
    pub const OIPF_USERPROFILE: crate::Name = crate::Name::new_unchecked("vnd.oipf.userprofile");
    /// `vnd.olpc-sugar`
    pub const OLPC_SUGAR: crate::Name = crate::Name::new_unchecked("vnd.olpc-sugar");
    /// `vnd.oma-scws-config`
    pub const OMA_SCWS_CONFIG: crate::Name = crate::Name::new_unchecked("vnd.oma-scws-config");
    /// `vnd.oma-scws-http-request`
    pub const OMA_SCWS_HTTP_REQUEST: crate::Name = crate::Name::new_unchecked("vnd.oma-scws-http-request");
    /// `vnd.oma-scws-http-response`
    pub const OMA_SCWS_HTTP_RESPONSE: crate::Name = crate::Name::new_unchecked("vnd.oma-scws-http-response");
    /// `vnd.oma.bcast.associated-procedure-parameter`
    pub const OMA_BCAST_ASSOCIATED_PROCEDURE_PARAMETER: crate::Name = crate::Name::new_unchecked("vnd.oma.bcast.associated-procedure-parameter");
    /// `vnd.oma.bcast.drm-trigger`
    pub const OMA_BCAST_DRM_TRIGGER: crate::Name = crate::Name::new_unchecked("vnd.oma.bcast.drm-trigger");
    /// `vnd.oma.bcast.imd`
    pub const OMA_BCAST_IMD: crate::Name = crate::Name::new_unchecked("vnd.oma.bcast.imd");
    /// `vnd.oma.bcast.ltkm`
    pub const OMA_BCAST_LTKM: crate::Name = crate::Name::new_unchecked("vnd.oma.bcast.ltkm");
    /// `vnd.oma.bcast.notification`
    pub const OMA_BCAST_NOTIFICATION: crate::Name = crate::Name::new_unchecked("vnd.oma.bcast.notification");
    /// `vnd.oma.bcast.provisioningtrigger`
    pub const OMA_BCAST_PROVISIONINGTRIGGER: crate::Name = crate::Name::new_unchecked("vnd.oma.bcast.provisioningtrigger");
    /// `vnd.oma.bcast.sgboot`
    pub const OMA_BCAST_SGBOOT: crate::Name = crate::Name::new_unchecked("vnd.oma.bcast.sgboot");
    /// `vnd.oma.bcast.sgdd`
    pub const OMA_BCAST_SGDD: crate::Name = crate::Name::new_unchecked("vnd.oma.bcast.sgdd");
    /// `vnd.oma.bcast.sgdu`
    pub const OMA_BCAST_SGDU: crate::Name = crate::Name::new_unchecked("vnd.oma.bcast.sgdu");
    /// `vnd.oma.bcast.simple-symbol-container`
    pub const OMA_BCAST_SIMPLE_SYMBOL_CONTAINER: crate::Name = crate::Name::new_unchecked("vnd.oma.bcast.simple-symbol-container");
    /// `vnd.oma.bcast.smartcard-trigger`
    pub const OMA_BCAST_SMARTCARD_TRIGGER: crate::Name = crate::Name::new_unchecked("vnd.oma.bcast.smartcard-trigger");
    /// `vnd.oma.bcast.sprov`
    pub const OMA_BCAST_SPROV: crate::Name = crate::Name::new_unchecked("vnd.oma.bcast.sprov");
    /// `vnd.oma.bcast.stkm`
    pub const OMA_BCAST_STKM: crate::Name = crate::Name::new_unchecked("vnd.oma.bcast.stkm");
    /// `vnd.oma.cab-address-book`
    pub const OMA_CAB_ADDRESS_BOOK: crate::Name = crate::Name::new_unchecked("vnd.oma.cab-address-book");
    /// `vnd.oma.cab-feature-handler`
    pub const OMA_CAB_FEATURE_HANDLER: crate::Name = crate::Name::new_unchecked("vnd.oma.cab-feature-handler");
    /// `vnd.oma.cab-pcc`
    pub const OMA_CAB_PCC: crate::Name = crate::Name::new_unchecked("vnd.oma.cab-pcc");
    /// `vnd.oma.cab-subs-invite`
    pub const OMA_CAB_SUBS_INVITE: crate::Name = crate::Name::new_unchecked("vnd.oma.cab-subs-invite");
    /// `vnd.oma.cab-user-prefs`
    pub const OMA_CAB_USER_PREFS: crate::Name = crate::Name::new_unchecked("vnd.oma.cab-user-prefs");
    /// `vnd.oma.dcd`
    pub const OMA_DCD: crate::Name = crate::Name::new_unchecked("vnd.oma.dcd");
    /// `vnd.oma.dcdc`
    pub const OMA_DCDC: crate::Name = crate::Name::new_unchecked("vnd.oma.dcdc");
    /// `vnd.oma.dd2`
    pub const OMA_DD2: crate::Name = crate::Name::new_unchecked("vnd.oma.dd2");
    /// `vnd.oma.drm.risd`
    pub const OMA_DRM_RISD: crate::Name = crate::Name::new_unchecked("vnd.oma.drm.risd");
    /// `vnd.oma.group-usage-list`
    pub const OMA_GROUP_USAGE_LIST: crate::Name = crate::Name::new_unchecked("vnd.oma.group-usage-list");
    /// `vnd.oma.lwm2m`
    pub const OMA_LWM2M: crate::Name = crate::Name::new_unchecked("vnd.oma.lwm2m");
    /// `vnd.oma.pal`
    pub const OMA_PAL: crate::Name = crate::Name::new_unchecked("vnd.oma.pal");
    /// `vnd.oma.poc.detailed-progress-report`
    pub const OMA_POC_DETAILED_PROGRESS_REPORT: crate::Name = crate::Name::new_unchecked("vnd.oma.poc.detailed-progress-report");
    /// `vnd.oma.poc.final-report`
    pub const OMA_POC_FINAL_REPORT: crate::Name = crate::Name::new_unchecked("vnd.oma.poc.final-report");
    /// `vnd.oma.poc.groups`
    pub const OMA_POC_GROUPS: crate::Name = crate::Name::new_unchecked("vnd.oma.poc.groups");
    /// `vnd.oma.poc.invocation-descriptor`
    pub const OMA_POC_INVOCATION_DESCRIPTOR: crate::Name = crate::Name::new_unchecked("vnd.oma.poc.invocation-descriptor");
    /// `vnd.oma.poc.optimized-progress-report`
    pub const OMA_POC_OPTIMIZED_PROGRESS_REPORT: crate::Name = crate::Name::new_unchecked("vnd.oma.poc.optimized-progress-report");
    /// `vnd.oma.push`
    pub const OMA_PUSH: crate::Name = crate::Name::new_unchecked("vnd.oma.push");
    /// `vnd.oma.scidm.messages`
    pub const OMA_SCIDM_MESSAGES: crate::Name = crate::Name::new_unchecked("vnd.oma.scidm.messages");
    /// `vnd.oma.xcap-directory`
    pub const OMA_XCAP_DIRECTORY: crate::Name = crate::Name::new_unchecked("vnd.oma.xcap-directory");
    /// `vnd.omads-email`
    pub const OMADS_EMAIL: crate::Name = crate::Name::new_unchecked("vnd.omads-email");
    /// `vnd.omads-file`
    pub const OMADS_FILE: crate::Name = crate::Name::new_unchecked("vnd.omads-file");
    /// `vnd.omads-folder`
    pub const OMADS_FOLDER: crate::Name = crate::Name::new_unchecked("vnd.omads-folder");
    /// `vnd.omaloc-supl-init`
    pub const OMALOC_SUPL_INIT: crate::Name = crate::Name::new_unchecked("vnd.omaloc-supl-init");
    /// `vnd.onepager`
    pub const ONEPAGER: crate::Name = crate::Name::new_unchecked("vnd.onepager");
    /// `vnd.onepagertamp`
    pub const ONEPAGERTAMP: crate::Name = crate::Name::new_unchecked("vnd.onepagertamp");
    /// `vnd.onepagertamx`
    pub const ONEPAGERTAMX: crate::Name = crate::Name::new_unchecked("vnd.onepagertamx");
    /// `vnd.onepagertat`
    pub const ONEPAGERTAT: crate::Name = crate::Name::new_unchecked("vnd.onepagertat");
    /// `vnd.onepagertatp`
    pub const ONEPAGERTATP: crate::Name = crate::Name::new_unchecked("vnd.onepagertatp");
    /// `vnd.onepagertatx`
    pub const ONEPAGERTATX: crate::Name = crate::Name::new_unchecked("vnd.onepagertatx");
    /// `vnd.openblox.game`
    pub const OPENBLOX_GAME: crate::Name = crate::Name::new_unchecked("vnd.openblox.game");
    /// `vnd.openblox.game-binary`
    pub const OPENBLOX_GAME_BINARY: crate::Name = crate::Name::new_unchecked("vnd.openblox.game-binary");
    /// `vnd.openeye.oeb`
    pub const OPENEYE_OEB: crate::Name = crate::Name::new_unchecked("vnd.openeye.oeb");
    /// `vnd.opengex`
    pub const OPENGEX: crate::Name = crate::Name::new_unchecked("vnd.opengex");
    /// `vnd.openstreetmap.data`
    pub const OPENSTREETMAP_DATA: crate::Name = crate::Name::new_unchecked("vnd.openstreetmap.data");
    /// `vnd.opentimestamps.ots`
    pub const OPENTIMESTAMPS_OTS: crate::Name = crate::Name::new_unchecked("vnd.opentimestamps.ots");
    /// `vnd.openxmlformats-officedocument.custom-properties`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_CUSTOM_PROPERTIES: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.custom-properties");
    /// `vnd.openxmlformats-officedocument.customXmlProperties`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_CUSTOM_XML_PROPERTIES: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.customXmlProperties");
    /// `vnd.openxmlformats-officedocument.drawing`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_DRAWING: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.drawing");
    /// `vnd.openxmlformats-officedocument.drawingml.chart`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_DRAWINGML_CHART: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.drawingml.chart");
    /// `vnd.openxmlformats-officedocument.drawingml.chartshapes`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_DRAWINGML_CHARTSHAPES: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.drawingml.chartshapes");
    /// `vnd.openxmlformats-officedocument.drawingml.diagramColors`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_DRAWINGML_DIAGRAM_COLORS: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.drawingml.diagramColors");
    /// `vnd.openxmlformats-officedocument.drawingml.diagramData`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_DRAWINGML_DIAGRAM_DATA: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.drawingml.diagramData");
    /// `vnd.openxmlformats-officedocument.drawingml.diagramLayout`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_DRAWINGML_DIAGRAM_LAYOUT: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.drawingml.diagramLayout");
    /// `vnd.openxmlformats-officedocument.drawingml.diagramStyle`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_DRAWINGML_DIAGRAM_STYLE: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.drawingml.diagramStyle");
    /// `vnd.openxmlformats-officedocument.extended-properties`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_EXTENDED_PROPERTIES: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.extended-properties");
    /// `vnd.openxmlformats-officedocument.presentationml.commentAuthors`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_COMMENT_AUTHORS: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.presentationml.commentAuthors");
    /// `vnd.openxmlformats-officedocument.presentationml.comments`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_COMMENTS: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.presentationml.comments");
    /// `vnd.openxmlformats-officedocument.presentationml.handoutMaster`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_HANDOUT_MASTER: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.presentationml.handoutMaster");
    /// `vnd.openxmlformats-officedocument.presentationml.notesMaster`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_NOTES_MASTER: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.presentationml.notesMaster");
    /// `vnd.openxmlformats-officedocument.presentationml.notesSlide`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_NOTES_SLIDE: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.presentationml.notesSlide");
    /// `vnd.openxmlformats-officedocument.presentationml.presentation`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_PRESENTATION: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.presentationml.presentation");
    /// `vnd.openxmlformats-officedocument.presentationml.presentation.main`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_PRESENTATION_MAIN: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.presentationml.presentation.main");
    /// `vnd.openxmlformats-officedocument.presentationml.presProps`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_PRES_PROPS: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.presentationml.presProps");
    /// `vnd.openxmlformats-officedocument.presentationml.slide`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_SLIDE: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.presentationml.slide");
    /// `vnd.openxmlformats-officedocument.presentationml.slideLayout`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_SLIDE_LAYOUT: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.presentationml.slideLayout");
    /// `vnd.openxmlformats-officedocument.presentationml.slideMaster`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_SLIDE_MASTER: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.presentationml.slideMaster");
    /// `vnd.openxmlformats-officedocument.presentationml.slideshow`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_SLIDESHOW: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.presentationml.slideshow");
    /// `vnd.openxmlformats-officedocument.presentationml.slideshow.main`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_SLIDESHOW_MAIN: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.presentationml.slideshow.main");
    /// `vnd.openxmlformats-officedocument.presentationml.slideUpdateInfo`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_SLIDE_UPDATE_INFO: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.presentationml.slideUpdateInfo");
    /// `vnd.openxmlformats-officedocument.presentationml.tableStyles`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_TABLE_STYLES: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.presentationml.tableStyles");
    /// `vnd.openxmlformats-officedocument.presentationml.tags`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_TAGS: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.presentationml.tags");
    /// `vnd.openxmlformats-officedocument.presentationml.template`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_TEMPLATE: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.presentationml.template");
    /// `vnd.openxmlformats-officedocument.presentationml.template.main`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_TEMPLATE_MAIN: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.presentationml.template.main");
    /// `vnd.openxmlformats-officedocument.presentationml.viewProps`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_VIEW_PROPS: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.presentationml.viewProps");
    /// `vnd.openxmlformats-officedocument.spreadsheetml.calcChain`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_CALC_CHAIN: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.spreadsheetml.calcChain");
    /// `vnd.openxmlformats-officedocument.spreadsheetml.chartsheet`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_CHARTSHEET: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.spreadsheetml.chartsheet");
    /// `vnd.openxmlformats-officedocument.spreadsheetml.comments`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_COMMENTS: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.spreadsheetml.comments");
    /// `vnd.openxmlformats-officedocument.spreadsheetml.connections`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_CONNECTIONS: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.spreadsheetml.connections");
    /// `vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_DIALOGSHEET: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet");
    /// `vnd.openxmlformats-officedocument.spreadsheetml.externalLink`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_EXTERNAL_LINK: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.spreadsheetml.externalLink");
    /// `vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_PIVOT_CACHE_DEFINITION: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition");
    /// `vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheRecords`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_PIVOT_CACHE_RECORDS: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheRecords");
    /// `vnd.openxmlformats-officedocument.spreadsheetml.pivotTable`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_PIVOT_TABLE: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.spreadsheetml.pivotTable");
    /// `vnd.openxmlformats-officedocument.spreadsheetml.queryTable`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_QUERY_TABLE: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.spreadsheetml.queryTable");
    /// `vnd.openxmlformats-officedocument.spreadsheetml.revisionHeaders`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_REVISION_HEADERS: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.spreadsheetml.revisionHeaders");
    /// `vnd.openxmlformats-officedocument.spreadsheetml.revisionLog`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_REVISION_LOG: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.spreadsheetml.revisionLog");
    /// `vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_SHARED_STRINGS: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings");
    /// `vnd.openxmlformats-officedocument.spreadsheetml.sheet`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_SHEET: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.spreadsheetml.sheet");
    /// `vnd.openxmlformats-officedocument.spreadsheetml.sheet.main`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_SHEET_MAIN: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.spreadsheetml.sheet.main");
    /// `vnd.openxmlformats-officedocument.spreadsheetml.sheetMetadata`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_SHEET_METADATA: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.spreadsheetml.sheetMetadata");
    /// `vnd.openxmlformats-officedocument.spreadsheetml.styles`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_STYLES: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.spreadsheetml.styles");
    /// `vnd.openxmlformats-officedocument.spreadsheetml.table`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_TABLE: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.spreadsheetml.table");
    /// `vnd.openxmlformats-officedocument.spreadsheetml.tableSingleCells`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_TABLE_SINGLE_CELLS: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.spreadsheetml.tableSingleCells");
    /// `vnd.openxmlformats-officedocument.spreadsheetml.template`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_TEMPLATE: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.spreadsheetml.template");
    /// `vnd.openxmlformats-officedocument.spreadsheetml.template.main`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_TEMPLATE_MAIN: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.spreadsheetml.template.main");
    /// `vnd.openxmlformats-officedocument.spreadsheetml.userNames`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_USER_NAMES: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.spreadsheetml.userNames");
    /// `vnd.openxmlformats-officedocument.spreadsheetml.volatileDependencies`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_VOLATILE_DEPENDENCIES: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.spreadsheetml.volatileDependencies");
    /// `vnd.openxmlformats-officedocument.spreadsheetml.worksheet`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_WORKSHEET: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.spreadsheetml.worksheet");
    /// `vnd.openxmlformats-officedocument.theme`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_THEME: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.theme");
    /// `vnd.openxmlformats-officedocument.themeOverride`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_THEME_OVERRIDE: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.themeOverride");
    /// `vnd.openxmlformats-officedocument.vmlDrawing`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_VML_DRAWING: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.vmlDrawing");
    /// `vnd.openxmlformats-officedocument.wordprocessingml.comments`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_COMMENTS: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.wordprocessingml.comments");
    /// `vnd.openxmlformats-officedocument.wordprocessingml.document`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_DOCUMENT: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.wordprocessingml.document");
    /// `vnd.openxmlformats-officedocument.wordprocessingml.document.glossary`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_DOCUMENT_GLOSSARY: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.wordprocessingml.document.glossary");
    /// `vnd.openxmlformats-officedocument.wordprocessingml.document.main`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_DOCUMENT_MAIN: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.wordprocessingml.document.main");
    /// `vnd.openxmlformats-officedocument.wordprocessingml.endnotes`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_ENDNOTES: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.wordprocessingml.endnotes");
    /// `vnd.openxmlformats-officedocument.wordprocessingml.fontTable`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_FONT_TABLE: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.wordprocessingml.fontTable");
    /// `vnd.openxmlformats-officedocument.wordprocessingml.footer`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_FOOTER: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.wordprocessingml.footer");
    /// `vnd.openxmlformats-officedocument.wordprocessingml.footnotes`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_FOOTNOTES: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.wordprocessingml.footnotes");
    /// `vnd.openxmlformats-officedocument.wordprocessingml.numbering`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_NUMBERING: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.wordprocessingml.numbering");
    /// `vnd.openxmlformats-officedocument.wordprocessingml.settings`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_SETTINGS: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.wordprocessingml.settings");
    /// `vnd.openxmlformats-officedocument.wordprocessingml.styles`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_STYLES: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.wordprocessingml.styles");
    /// `vnd.openxmlformats-officedocument.wordprocessingml.template`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_TEMPLATE: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.wordprocessingml.template");
    /// `vnd.openxmlformats-officedocument.wordprocessingml.template.main`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_TEMPLATE_MAIN: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.wordprocessingml.template.main");
    /// `vnd.openxmlformats-officedocument.wordprocessingml.webSettings`
    pub const OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_WEB_SETTINGS: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-officedocument.wordprocessingml.webSettings");
    /// `vnd.openxmlformats-package.core-properties`
    pub const OPENXMLFORMATS_PACKAGE_CORE_PROPERTIES: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-package.core-properties");
    /// `vnd.openxmlformats-package.digital-signature-xmlsignature`
    pub const OPENXMLFORMATS_PACKAGE_DIGITAL_SIGNATURE_XMLSIGNATURE: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-package.digital-signature-xmlsignature");
    /// `vnd.openxmlformats-package.relationships`
    pub const OPENXMLFORMATS_PACKAGE_RELATIONSHIPS: crate::Name = crate::Name::new_unchecked("vnd.openxmlformats-package.relationships");
    /// `vnd.oracle.resource`
    pub const ORACLE_RESOURCE: crate::Name = crate::Name::new_unchecked("vnd.oracle.resource");
    /// `vnd.orange.indata`
    pub const ORANGE_INDATA: crate::Name = crate::Name::new_unchecked("vnd.orange.indata");
    /// `vnd.osa.netdeploy`
    pub const OSA_NETDEPLOY: crate::Name = crate::Name::new_unchecked("vnd.osa.netdeploy");
    /// `vnd.osgeo.mapguide.package`
    pub const OSGEO_MAPGUIDE_PACKAGE: crate::Name = crate::Name::new_unchecked("vnd.osgeo.mapguide.package");
    /// `vnd.osgi.bundle`
    pub const OSGI_BUNDLE: crate::Name = crate::Name::new_unchecked("vnd.osgi.bundle");
    /// `vnd.osgi.dp`
    pub const OSGI_DP: crate::Name = crate::Name::new_unchecked("vnd.osgi.dp");
    /// `vnd.osgi.subsystem`
    pub const OSGI_SUBSYSTEM: crate::Name = crate::Name::new_unchecked("vnd.osgi.subsystem");
    /// `vnd.otps.ct-kip`
    pub const OTPS_CT_KIP: crate::Name = crate::Name::new_unchecked("vnd.otps.ct-kip");
    /// `vnd.oxli.countgraph`
    pub const OXLI_COUNTGRAPH: crate::Name = crate::Name::new_unchecked("vnd.oxli.countgraph");
    /// `vnd.pagerduty`
    pub const PAGERDUTY: crate::Name = crate::Name::new_unchecked("vnd.pagerduty");
    /// `vnd.palm`
    pub const PALM: crate::Name = crate::Name::new_unchecked("vnd.palm");
    /// `vnd.panoply`
    pub const PANOPLY: crate::Name = crate::Name::new_unchecked("vnd.panoply");
    /// `vnd.paos.xml`
    pub const PAOS_XML: crate::Name = crate::Name::new_unchecked("vnd.paos.xml");
    /// `vnd.parasolid.transmit.binary`
    pub const PARASOLID_TRANSMIT_BINARY: crate::Name = crate::Name::new_unchecked("vnd.parasolid.transmit.binary");
    /// `vnd.parasolid.transmit.text`
    pub const PARASOLID_TRANSMIT_TEXT: crate::Name = crate::Name::new_unchecked("vnd.parasolid.transmit.text");
    /// `vnd.patentdive`
    pub const PATENTDIVE: crate::Name = crate::Name::new_unchecked("vnd.patentdive");
    /// `vnd.patientecommsdoc`
    pub const PATIENTECOMMSDOC: crate::Name = crate::Name::new_unchecked("vnd.patientecommsdoc");
    /// `vnd.pawaafile`
    pub const PAWAAFILE: crate::Name = crate::Name::new_unchecked("vnd.pawaafile");
    /// `vnd.pco.b16`
    pub const PCO_B16: crate::Name = crate::Name::new_unchecked("vnd.pco.b16");
    /// `vnd.pcos`
    pub const PCOS: crate::Name = crate::Name::new_unchecked("vnd.pcos");
    /// `vnd.pg.format`
    pub const PG_FORMAT: crate::Name = crate::Name::new_unchecked("vnd.pg.format");
    /// `vnd.pg.osasli`
    pub const PG_OSASLI: crate::Name = crate::Name::new_unchecked("vnd.pg.osasli");
    /// `vnd.piaccess.application-licence`
    pub const PIACCESS_APPLICATION_LICENCE: crate::Name = crate::Name::new_unchecked("vnd.piaccess.application-licence");
    /// `vnd.picsel`
    pub const PICSEL: crate::Name = crate::Name::new_unchecked("vnd.picsel");
    /// `vnd.pmi.widget`
    pub const PMI_WIDGET: crate::Name = crate::Name::new_unchecked("vnd.pmi.widget");
    /// `vnd.poc.group-advertisement`
    pub const POC_GROUP_ADVERTISEMENT: crate::Name = crate::Name::new_unchecked("vnd.poc.group-advertisement");
    /// `vnd.pocketlearn`
    pub const POCKETLEARN: crate::Name = crate::Name::new_unchecked("vnd.pocketlearn");
    /// `vnd.powerbuilder6`
    pub const POWERBUILDER6: crate::Name = crate::Name::new_unchecked("vnd.powerbuilder6");
    /// `vnd.powerbuilder6-s`
    pub const POWERBUILDER6_S: crate::Name = crate::Name::new_unchecked("vnd.powerbuilder6-s");
    /// `vnd.powerbuilder7`
    pub const POWERBUILDER7: crate::Name = crate::Name::new_unchecked("vnd.powerbuilder7");
    /// `vnd.powerbuilder7-s`
    pub const POWERBUILDER7_S: crate::Name = crate::Name::new_unchecked("vnd.powerbuilder7-s");
    /// `vnd.powerbuilder75`
    pub const POWERBUILDER75: crate::Name = crate::Name::new_unchecked("vnd.powerbuilder75");
    /// `vnd.powerbuilder75-s`
    pub const POWERBUILDER75_S: crate::Name = crate::Name::new_unchecked("vnd.powerbuilder75-s");
    /// `vnd.preminet`
    pub const PREMINET: crate::Name = crate::Name::new_unchecked("vnd.preminet");
    /// `vnd.presonus.multitrack`
    pub const PRESONUS_MULTITRACK: crate::Name = crate::Name::new_unchecked("vnd.presonus.multitrack");
    /// `vnd.previewsystems.box`
    pub const PREVIEWSYSTEMS_BOX: crate::Name = crate::Name::new_unchecked("vnd.previewsystems.box");
    /// `vnd.proteus.magazine`
    pub const PROTEUS_MAGAZINE: crate::Name = crate::Name::new_unchecked("vnd.proteus.magazine");
    /// `vnd.psfs`
    pub const PSFS: crate::Name = crate::Name::new_unchecked("vnd.psfs");
    /// `vnd.publishare-delta-tree`
    pub const PUBLISHARE_DELTA_TREE: crate::Name = crate::Name::new_unchecked("vnd.publishare-delta-tree");
    /// `vnd.pvi.ptid1`
    pub const PVI_PTID1: crate::Name = crate::Name::new_unchecked("vnd.pvi.ptid1");
    /// `vnd.pwg-multiplexed`
    pub const PWG_MULTIPLEXED: crate::Name = crate::Name::new_unchecked("vnd.pwg-multiplexed");
    /// `vnd.pwg-xhtml-print`
    pub const PWG_XHTML_PRINT: crate::Name = crate::Name::new_unchecked("vnd.pwg-xhtml-print");
    /// `vnd.pytha.pyox`
    pub const PYTHA_PYOX: crate::Name = crate::Name::new_unchecked("vnd.pytha.pyox");
    /// `vnd.qcelp`
    pub const QCELP: crate::Name = crate::Name::new_unchecked("vnd.qcelp");
    /// `vnd.qualcomm.brew-app-res`
    pub const QUALCOMM_BREW_APP_RES: crate::Name = crate::Name::new_unchecked("vnd.qualcomm.brew-app-res");
    /// `vnd.quarantainenet`
    pub const QUARANTAINENET: crate::Name = crate::Name::new_unchecked("vnd.quarantainenet");
    /// `vnd.Quark.QuarkXPress`
    pub const QUARK_QUARK_XPRESS: crate::Name = crate::Name::new_unchecked("vnd.Quark.QuarkXPress");
    /// `vnd.quobject-quoxdocument`
    pub const QUOBJECT_QUOXDOCUMENT: crate::Name = crate::Name::new_unchecked("vnd.quobject-quoxdocument");
    /// `vnd.radgamettools.bink`
    pub const RADGAMETTOOLS_BINK: crate::Name = crate::Name::new_unchecked("vnd.radgamettools.bink");
    /// `vnd.radgamettools.smacker`
    pub const RADGAMETTOOLS_SMACKER: crate::Name = crate::Name::new_unchecked("vnd.radgamettools.smacker");
    /// `vnd.radiance`
    pub const RADIANCE: crate::Name = crate::Name::new_unchecked("vnd.radiance");
    /// `vnd.radisys.moml`
    pub const RADISYS_MOML: crate::Name = crate::Name::new_unchecked("vnd.radisys.moml");
    /// `vnd.radisys.msml`
    pub const RADISYS_MSML: crate::Name = crate::Name::new_unchecked("vnd.radisys.msml");
    /// `vnd.radisys.msml-audit`
    pub const RADISYS_MSML_AUDIT: crate::Name = crate::Name::new_unchecked("vnd.radisys.msml-audit");
    /// `vnd.radisys.msml-audit-conf`
    pub const RADISYS_MSML_AUDIT_CONF: crate::Name = crate::Name::new_unchecked("vnd.radisys.msml-audit-conf");
    /// `vnd.radisys.msml-audit-conn`
    pub const RADISYS_MSML_AUDIT_CONN: crate::Name = crate::Name::new_unchecked("vnd.radisys.msml-audit-conn");
    /// `vnd.radisys.msml-audit-dialog`
    pub const RADISYS_MSML_AUDIT_DIALOG: crate::Name = crate::Name::new_unchecked("vnd.radisys.msml-audit-dialog");
    /// `vnd.radisys.msml-audit-stream`
    pub const RADISYS_MSML_AUDIT_STREAM: crate::Name = crate::Name::new_unchecked("vnd.radisys.msml-audit-stream");
    /// `vnd.radisys.msml-basic-layout`
    pub const RADISYS_MSML_BASIC_LAYOUT: crate::Name = crate::Name::new_unchecked("vnd.radisys.msml-basic-layout");
    /// `vnd.radisys.msml-conf`
    pub const RADISYS_MSML_CONF: crate::Name = crate::Name::new_unchecked("vnd.radisys.msml-conf");
    /// `vnd.radisys.msml-dialog`
    pub const RADISYS_MSML_DIALOG: crate::Name = crate::Name::new_unchecked("vnd.radisys.msml-dialog");
    /// `vnd.radisys.msml-dialog-base`
    pub const RADISYS_MSML_DIALOG_BASE: crate::Name = crate::Name::new_unchecked("vnd.radisys.msml-dialog-base");
    /// `vnd.radisys.msml-dialog-fax-detect`
    pub const RADISYS_MSML_DIALOG_FAX_DETECT: crate::Name = crate::Name::new_unchecked("vnd.radisys.msml-dialog-fax-detect");
    /// `vnd.radisys.msml-dialog-fax-sendrecv`
    pub const RADISYS_MSML_DIALOG_FAX_SENDRECV: crate::Name = crate::Name::new_unchecked("vnd.radisys.msml-dialog-fax-sendrecv");
    /// `vnd.radisys.msml-dialog-group`
    pub const RADISYS_MSML_DIALOG_GROUP: crate::Name = crate::Name::new_unchecked("vnd.radisys.msml-dialog-group");
    /// `vnd.radisys.msml-dialog-speech`
    pub const RADISYS_MSML_DIALOG_SPEECH: crate::Name = crate::Name::new_unchecked("vnd.radisys.msml-dialog-speech");
    /// `vnd.radisys.msml-dialog-transform`
    pub const RADISYS_MSML_DIALOG_TRANSFORM: crate::Name = crate::Name::new_unchecked("vnd.radisys.msml-dialog-transform");
    /// `vnd.rainstor.data`
    pub const RAINSTOR_DATA: crate::Name = crate::Name::new_unchecked("vnd.rainstor.data");
    /// `vnd.rapid`
    pub const RAPID: crate::Name = crate::Name::new_unchecked("vnd.rapid");
    /// `vnd.rar`
    pub const RAR: crate::Name = crate::Name::new_unchecked("vnd.rar");
    /// `vnd.realvnc.bed`
    pub const REALVNC_BED: crate::Name = crate::Name::new_unchecked("vnd.realvnc.bed");
    /// `vnd.recordare.musicxml`
    pub const RECORDARE_MUSICXML: crate::Name = crate::Name::new_unchecked("vnd.recordare.musicxml");
    /// `vnd.RenLearn.rlprint`
    pub const REN_LEARN_RLPRINT: crate::Name = crate::Name::new_unchecked("vnd.RenLearn.rlprint");
    /// `vnd.resilient.logic`
    pub const RESILIENT_LOGIC: crate::Name = crate::Name::new_unchecked("vnd.resilient.logic");
    /// `vnd.restful`
    pub const RESTFUL: crate::Name = crate::Name::new_unchecked("vnd.restful");
    /// `vnd.rhetorex.32kadpcm`
    pub const RHETOREX_32KADPCM: crate::Name = crate::Name::new_unchecked("vnd.rhetorex.32kadpcm");
    /// `vnd.rig.cryptonote`
    pub const RIG_CRYPTONOTE: crate::Name = crate::Name::new_unchecked("vnd.rig.cryptonote");
    /// `vnd.rip`
    pub const RIP: crate::Name = crate::Name::new_unchecked("vnd.rip");
    /// `vnd.rosette.annotated-data-model`
    pub const ROSETTE_ANNOTATED_DATA_MODEL: crate::Name = crate::Name::new_unchecked("vnd.rosette.annotated-data-model");
    /// `vnd.route66.link66`
    pub const ROUTE66_LINK66: crate::Name = crate::Name::new_unchecked("vnd.route66.link66");
    /// `vnd.rs-274x`
    pub const RS_274X: crate::Name = crate::Name::new_unchecked("vnd.rs-274x");
    /// `vnd.ruckus.download`
    pub const RUCKUS_DOWNLOAD: crate::Name = crate::Name::new_unchecked("vnd.ruckus.download");
    /// `vnd.s3sms`
    pub const S3SMS: crate::Name = crate::Name::new_unchecked("vnd.s3sms");
    /// `vnd.sailingtracker.track`
    pub const SAILINGTRACKER_TRACK: crate::Name = crate::Name::new_unchecked("vnd.sailingtracker.track");
    /// `vnd.sap.vds`
    pub const SAP_VDS: crate::Name = crate::Name::new_unchecked("vnd.sap.vds");
    /// `vnd.sar`
    pub const SAR: crate::Name = crate::Name::new_unchecked("vnd.sar");
    /// `vnd.sbm.cid`
    pub const SBM_CID: crate::Name = crate::Name::new_unchecked("vnd.sbm.cid");
    /// `vnd.sbm.mid2`
    pub const SBM_MID2: crate::Name = crate::Name::new_unchecked("vnd.sbm.mid2");
    /// `vnd.scribus`
    pub const SCRIBUS: crate::Name = crate::Name::new_unchecked("vnd.scribus");
    /// `vnd.sealed.3df`
    pub const SEALED_3DF: crate::Name = crate::Name::new_unchecked("vnd.sealed.3df");
    /// `vnd.sealed.csf`
    pub const SEALED_CSF: crate::Name = crate::Name::new_unchecked("vnd.sealed.csf");
    /// `vnd.sealed.doc`
    pub const SEALED_DOC: crate::Name = crate::Name::new_unchecked("vnd.sealed.doc");
    /// `vnd.sealed.eml`
    pub const SEALED_EML: crate::Name = crate::Name::new_unchecked("vnd.sealed.eml");
    /// `vnd.sealed.mht`
    pub const SEALED_MHT: crate::Name = crate::Name::new_unchecked("vnd.sealed.mht");
    /// `vnd.sealed.mpeg1`
    pub const SEALED_MPEG1: crate::Name = crate::Name::new_unchecked("vnd.sealed.mpeg1");
    /// `vnd.sealed.mpeg4`
    pub const SEALED_MPEG4: crate::Name = crate::Name::new_unchecked("vnd.sealed.mpeg4");
    /// `vnd.sealed.net`
    pub const SEALED_NET: crate::Name = crate::Name::new_unchecked("vnd.sealed.net");
    /// `vnd.sealed.png`
    pub const SEALED_PNG: crate::Name = crate::Name::new_unchecked("vnd.sealed.png");
    /// `vnd.sealed.ppt`
    pub const SEALED_PPT: crate::Name = crate::Name::new_unchecked("vnd.sealed.ppt");
    /// `vnd.sealed.swf`
    pub const SEALED_SWF: crate::Name = crate::Name::new_unchecked("vnd.sealed.swf");
    /// `vnd.sealed.tiff`
    pub const SEALED_TIFF: crate::Name = crate::Name::new_unchecked("vnd.sealed.tiff");
    /// `vnd.sealed.xls`
    pub const SEALED_XLS: crate::Name = crate::Name::new_unchecked("vnd.sealed.xls");
    /// `vnd.sealedmedia.softseal.gif`
    pub const SEALEDMEDIA_SOFTSEAL_GIF: crate::Name = crate::Name::new_unchecked("vnd.sealedmedia.softseal.gif");
    /// `vnd.sealedmedia.softseal.html`
    pub const SEALEDMEDIA_SOFTSEAL_HTML: crate::Name = crate::Name::new_unchecked("vnd.sealedmedia.softseal.html");
    /// `vnd.sealedmedia.softseal.jpg`
    pub const SEALEDMEDIA_SOFTSEAL_JPG: crate::Name = crate::Name::new_unchecked("vnd.sealedmedia.softseal.jpg");
    /// `vnd.sealedmedia.softseal.mov`
    pub const SEALEDMEDIA_SOFTSEAL_MOV: crate::Name = crate::Name::new_unchecked("vnd.sealedmedia.softseal.mov");
    /// `vnd.sealedmedia.softseal.mpeg`
    pub const SEALEDMEDIA_SOFTSEAL_MPEG: crate::Name = crate::Name::new_unchecked("vnd.sealedmedia.softseal.mpeg");
    /// `vnd.sealedmedia.softseal.pdf`
    pub const SEALEDMEDIA_SOFTSEAL_PDF: crate::Name = crate::Name::new_unchecked("vnd.sealedmedia.softseal.pdf");
    /// `vnd.seemail`
    pub const SEEMAIL: crate::Name = crate::Name::new_unchecked("vnd.seemail");
    /// `vnd.seis`
    pub const SEIS: crate::Name = crate::Name::new_unchecked("vnd.seis");
    /// `vnd.sema`
    pub const SEMA: crate::Name = crate::Name::new_unchecked("vnd.sema");
    /// `vnd.semd`
    pub const SEMD: crate::Name = crate::Name::new_unchecked("vnd.semd");
    /// `vnd.semf`
    pub const SEMF: crate::Name = crate::Name::new_unchecked("vnd.semf");
    /// `vnd.senx.warpscript`
    pub const SENX_WARPSCRIPT: crate::Name = crate::Name::new_unchecked("vnd.senx.warpscript");
    /// `vnd.shade-save-file`
    pub const SHADE_SAVE_FILE: crate::Name = crate::Name::new_unchecked("vnd.shade-save-file");
    /// `vnd.shana.informed.formdata`
    pub const SHANA_INFORMED_FORMDATA: crate::Name = crate::Name::new_unchecked("vnd.shana.informed.formdata");
    /// `vnd.shana.informed.formtemplate`
    pub const SHANA_INFORMED_FORMTEMPLATE: crate::Name = crate::Name::new_unchecked("vnd.shana.informed.formtemplate");
    /// `vnd.shana.informed.interchange`
    pub const SHANA_INFORMED_INTERCHANGE: crate::Name = crate::Name::new_unchecked("vnd.shana.informed.interchange");
    /// `vnd.shana.informed.package`
    pub const SHANA_INFORMED_PACKAGE: crate::Name = crate::Name::new_unchecked("vnd.shana.informed.package");
    /// `vnd.shootproof`
    pub const SHOOTPROOF: crate::Name = crate::Name::new_unchecked("vnd.shootproof");
    /// `vnd.shopkick`
    pub const SHOPKICK: crate::Name = crate::Name::new_unchecked("vnd.shopkick");
    /// `vnd.shp`
    pub const SHP: crate::Name = crate::Name::new_unchecked("vnd.shp");
    /// `vnd.shx`
    pub const SHX: crate::Name = crate::Name::new_unchecked("vnd.shx");
    /// `vnd.si.uricatalogue`
    pub const SI_URICATALOGUE: crate::Name = crate::Name::new_unchecked("vnd.si.uricatalogue");
    /// `vnd.sigrok.session`
    pub const SIGROK_SESSION: crate::Name = crate::Name::new_unchecked("vnd.sigrok.session");
    /// `vnd.SimTech-MindMapper`
    pub const SIM_TECH_MIND_MAPPER: crate::Name = crate::Name::new_unchecked("vnd.SimTech-MindMapper");
    /// `vnd.siren`
    pub const SIREN: crate::Name = crate::Name::new_unchecked("vnd.siren");
    /// `vnd.smaf`
    pub const SMAF: crate::Name = crate::Name::new_unchecked("vnd.smaf");
    /// `vnd.smart.notebook`
    pub const SMART_NOTEBOOK: crate::Name = crate::Name::new_unchecked("vnd.smart.notebook");
    /// `vnd.smart.teacher`
    pub const SMART_TEACHER: crate::Name = crate::Name::new_unchecked("vnd.smart.teacher");
    /// `vnd.snesdev-page-table`
    pub const SNESDEV_PAGE_TABLE: crate::Name = crate::Name::new_unchecked("vnd.snesdev-page-table");
    /// `vnd.software602.filler.form`
    pub const SOFTWARE602_FILLER_FORM: crate::Name = crate::Name::new_unchecked("vnd.software602.filler.form");
    /// `vnd.software602.filler.form-xml-zip`
    pub const SOFTWARE602_FILLER_FORM_XML_ZIP: crate::Name = crate::Name::new_unchecked("vnd.software602.filler.form-xml-zip");
    /// `vnd.solent.sdkm`
    pub const SOLENT_SDKM: crate::Name = crate::Name::new_unchecked("vnd.solent.sdkm");
    /// `vnd.sosi`
    pub const SOSI: crate::Name = crate::Name::new_unchecked("vnd.sosi");
    /// `vnd.spotfire.dxp`
    pub const SPOTFIRE_DXP: crate::Name = crate::Name::new_unchecked("vnd.spotfire.dxp");
    /// `vnd.spotfire.sfs`
    pub const SPOTFIRE_SFS: crate::Name = crate::Name::new_unchecked("vnd.spotfire.sfs");
    /// `vnd.sqlite3`
    pub const SQLITE3: crate::Name = crate::Name::new_unchecked("vnd.sqlite3");
    /// `vnd.sss-cod`
    pub const SSS_COD: crate::Name = crate::Name::new_unchecked("vnd.sss-cod");
    /// `vnd.sss-dtf`
    pub const SSS_DTF: crate::Name = crate::Name::new_unchecked("vnd.sss-dtf");
    /// `vnd.sss-ntf`
    pub const SSS_NTF: crate::Name = crate::Name::new_unchecked("vnd.sss-ntf");
    /// `vnd.stepmania.package`
    pub const STEPMANIA_PACKAGE: crate::Name = crate::Name::new_unchecked("vnd.stepmania.package");
    /// `vnd.stepmania.stepchart`
    pub const STEPMANIA_STEPCHART: crate::Name = crate::Name::new_unchecked("vnd.stepmania.stepchart");
    /// `vnd.street-stream`
    pub const STREET_STREAM: crate::Name = crate::Name::new_unchecked("vnd.street-stream");
    /// `vnd.sun.j2me.app-descriptor`
    pub const SUN_J2ME_APP_DESCRIPTOR: crate::Name = crate::Name::new_unchecked("vnd.sun.j2me.app-descriptor");
    /// `vnd.sun.wadl`
    pub const SUN_WADL: crate::Name = crate::Name::new_unchecked("vnd.sun.wadl");
    /// `vnd.sus-calendar`
    pub const SUS_CALENDAR: crate::Name = crate::Name::new_unchecked("vnd.sus-calendar");
    /// `vnd.svd`
    pub const SVD: crate::Name = crate::Name::new_unchecked("vnd.svd");
    /// `vnd.svf`
    pub const SVF: crate::Name = crate::Name::new_unchecked("vnd.svf");
    /// `vnd.swiftview-ics`
    pub const SWIFTVIEW_ICS: crate::Name = crate::Name::new_unchecked("vnd.swiftview-ics");
    /// `vnd.sycle`
    pub const SYCLE: crate::Name = crate::Name::new_unchecked("vnd.sycle");
    /// `vnd.syft`
    pub const SYFT: crate::Name = crate::Name::new_unchecked("vnd.syft");
    /// `vnd.syncml`
    pub const SYNCML: crate::Name = crate::Name::new_unchecked("vnd.syncml");
    /// `vnd.syncml.dm`
    pub const SYNCML_DM: crate::Name = crate::Name::new_unchecked("vnd.syncml.dm");
    /// `vnd.syncml.dm.notification`
    pub const SYNCML_DM_NOTIFICATION: crate::Name = crate::Name::new_unchecked("vnd.syncml.dm.notification");
    /// `vnd.syncml.dmddf`
    pub const SYNCML_DMDDF: crate::Name = crate::Name::new_unchecked("vnd.syncml.dmddf");
    /// `vnd.syncml.dmtnds`
    pub const SYNCML_DMTNDS: crate::Name = crate::Name::new_unchecked("vnd.syncml.dmtnds");
    /// `vnd.syncml.ds.notification`
    pub const SYNCML_DS_NOTIFICATION: crate::Name = crate::Name::new_unchecked("vnd.syncml.ds.notification");
    /// `vnd.tableschema`
    pub const TABLESCHEMA: crate::Name = crate::Name::new_unchecked("vnd.tableschema");
    /// `vnd.tao.intent-module-archive`
    pub const TAO_INTENT_MODULE_ARCHIVE: crate::Name = crate::Name::new_unchecked("vnd.tao.intent-module-archive");
    /// `vnd.tcpdump.pcap`
    pub const TCPDUMP_PCAP: crate::Name = crate::Name::new_unchecked("vnd.tcpdump.pcap");
    /// `vnd.tencent.tap`
    pub const TENCENT_TAP: crate::Name = crate::Name::new_unchecked("vnd.tencent.tap");
    /// `vnd.think-cell.ppttc`
    pub const THINK_CELL_PPTTC: crate::Name = crate::Name::new_unchecked("vnd.think-cell.ppttc");
    /// `vnd.tmd.mediaflex.api`
    pub const TMD_MEDIAFLEX_API: crate::Name = crate::Name::new_unchecked("vnd.tmd.mediaflex.api");
    /// `vnd.tml`
    pub const TML: crate::Name = crate::Name::new_unchecked("vnd.tml");
    /// `vnd.tmobile-livetv`
    pub const TMOBILE_LIVETV: crate::Name = crate::Name::new_unchecked("vnd.tmobile-livetv");
    /// `vnd.tri.onesource`
    pub const TRI_ONESOURCE: crate::Name = crate::Name::new_unchecked("vnd.tri.onesource");
    /// `vnd.trid.tpt`
    pub const TRID_TPT: crate::Name = crate::Name::new_unchecked("vnd.trid.tpt");
    /// `vnd.triscape.mxs`
    pub const TRISCAPE_MXS: crate::Name = crate::Name::new_unchecked("vnd.triscape.mxs");
    /// `vnd.trolltech.linguist`
    pub const TROLLTECH_LINGUIST: crate::Name = crate::Name::new_unchecked("vnd.trolltech.linguist");
    /// `vnd.trueapp`
    pub const TRUEAPP: crate::Name = crate::Name::new_unchecked("vnd.trueapp");
    /// `vnd.truedoc`
    pub const TRUEDOC: crate::Name = crate::Name::new_unchecked("vnd.truedoc");
    /// `vnd.ubisoft.webplayer`
    pub const UBISOFT_WEBPLAYER: crate::Name = crate::Name::new_unchecked("vnd.ubisoft.webplayer");
    /// `vnd.ufdl`
    pub const UFDL: crate::Name = crate::Name::new_unchecked("vnd.ufdl");
    /// `vnd.uiq.theme`
    pub const UIQ_THEME: crate::Name = crate::Name::new_unchecked("vnd.uiq.theme");
    /// `vnd.umajin`
    pub const UMAJIN: crate::Name = crate::Name::new_unchecked("vnd.umajin");
    /// `vnd.unity`
    pub const UNITY: crate::Name = crate::Name::new_unchecked("vnd.unity");
    /// `vnd.uoml`
    pub const UOML: crate::Name = crate::Name::new_unchecked("vnd.uoml");
    /// `vnd.uplanet.alert`
    pub const UPLANET_ALERT: crate::Name = crate::Name::new_unchecked("vnd.uplanet.alert");
    /// `vnd.uplanet.alert-wbxml`
    pub const UPLANET_ALERT_WBXML: crate::Name = crate::Name::new_unchecked("vnd.uplanet.alert-wbxml");
    /// `vnd.uplanet.bearer-choice`
    pub const UPLANET_BEARER_CHOICE: crate::Name = crate::Name::new_unchecked("vnd.uplanet.bearer-choice");
    /// `vnd.uplanet.bearer-choice-wbxml`
    pub const UPLANET_BEARER_CHOICE_WBXML: crate::Name = crate::Name::new_unchecked("vnd.uplanet.bearer-choice-wbxml");
    /// `vnd.uplanet.cacheop`
    pub const UPLANET_CACHEOP: crate::Name = crate::Name::new_unchecked("vnd.uplanet.cacheop");
    /// `vnd.uplanet.cacheop-wbxml`
    pub const UPLANET_CACHEOP_WBXML: crate::Name = crate::Name::new_unchecked("vnd.uplanet.cacheop-wbxml");
    /// `vnd.uplanet.channel`
    pub const UPLANET_CHANNEL: crate::Name = crate::Name::new_unchecked("vnd.uplanet.channel");
    /// `vnd.uplanet.channel-wbxml`
    pub const UPLANET_CHANNEL_WBXML: crate::Name = crate::Name::new_unchecked("vnd.uplanet.channel-wbxml");
    /// `vnd.uplanet.list`
    pub const UPLANET_LIST: crate::Name = crate::Name::new_unchecked("vnd.uplanet.list");
    /// `vnd.uplanet.list-wbxml`
    pub const UPLANET_LIST_WBXML: crate::Name = crate::Name::new_unchecked("vnd.uplanet.list-wbxml");
    /// `vnd.uplanet.listcmd`
    pub const UPLANET_LISTCMD: crate::Name = crate::Name::new_unchecked("vnd.uplanet.listcmd");
    /// `vnd.uplanet.listcmd-wbxml`
    pub const UPLANET_LISTCMD_WBXML: crate::Name = crate::Name::new_unchecked("vnd.uplanet.listcmd-wbxml");
    /// `vnd.uplanet.signal`
    pub const UPLANET_SIGNAL: crate::Name = crate::Name::new_unchecked("vnd.uplanet.signal");
    /// `vnd.uri-map`
    pub const URI_MAP: crate::Name = crate::Name::new_unchecked("vnd.uri-map");
    /// `vnd.usdz`
    pub const USDZ: crate::Name = crate::Name::new_unchecked("vnd.usdz");
    /// `vnd.uvvu.mp4`
    pub const UVVU_MP4: crate::Name = crate::Name::new_unchecked("vnd.uvvu.mp4");
    /// `vnd.valve.source.compiled-map`
    pub const VALVE_SOURCE_COMPILED_MAP: crate::Name = crate::Name::new_unchecked("vnd.valve.source.compiled-map");
    /// `vnd.valve.source.material`
    pub const VALVE_SOURCE_MATERIAL: crate::Name = crate::Name::new_unchecked("vnd.valve.source.material");
    /// `vnd.valve.source.texture`
    pub const VALVE_SOURCE_TEXTURE: crate::Name = crate::Name::new_unchecked("vnd.valve.source.texture");
    /// `vnd.vcx`
    pub const VCX: crate::Name = crate::Name::new_unchecked("vnd.vcx");
    /// `vnd.vd-study`
    pub const VD_STUDY: crate::Name = crate::Name::new_unchecked("vnd.vd-study");
    /// `vnd.vectorworks`
    pub const VECTORWORKS: crate::Name = crate::Name::new_unchecked("vnd.vectorworks");
    /// `vnd.vel`
    pub const VEL: crate::Name = crate::Name::new_unchecked("vnd.vel");
    /// `vnd.verimatrix.vcas`
    pub const VERIMATRIX_VCAS: crate::Name = crate::Name::new_unchecked("vnd.verimatrix.vcas");
    /// `vnd.veritone.aion`
    pub const VERITONE_AION: crate::Name = crate::Name::new_unchecked("vnd.veritone.aion");
    /// `vnd.veryant.thin`
    pub const VERYANT_THIN: crate::Name = crate::Name::new_unchecked("vnd.veryant.thin");
    /// `vnd.ves.encrypted`
    pub const VES_ENCRYPTED: crate::Name = crate::Name::new_unchecked("vnd.ves.encrypted");
    /// `vnd.vidsoft.vidconference`
    pub const VIDSOFT_VIDCONFERENCE: crate::Name = crate::Name::new_unchecked("vnd.vidsoft.vidconference");
    /// `vnd.visio`
    pub const VISIO: crate::Name = crate::Name::new_unchecked("vnd.visio");
    /// `vnd.visionary`
    pub const VISIONARY: crate::Name = crate::Name::new_unchecked("vnd.visionary");
    /// `vnd.vividence.scriptfile`
    pub const VIVIDENCE_SCRIPTFILE: crate::Name = crate::Name::new_unchecked("vnd.vividence.scriptfile");
    /// `vnd.vivo`
    pub const VIVO: crate::Name = crate::Name::new_unchecked("vnd.vivo");
    /// `vnd.vmx.cvsd`
    pub const VMX_CVSD: crate::Name = crate::Name::new_unchecked("vnd.vmx.cvsd");
    /// `vnd.vsf`
    pub const VSF: crate::Name = crate::Name::new_unchecked("vnd.vsf");
    /// `vnd.vtu`
    pub const VTU: crate::Name = crate::Name::new_unchecked("vnd.vtu");
    /// `vnd.wap.si`
    pub const WAP_SI: crate::Name = crate::Name::new_unchecked("vnd.wap.si");
    /// `vnd.wap.sic`
    pub const WAP_SIC: crate::Name = crate::Name::new_unchecked("vnd.wap.sic");
    /// `vnd.wap.sl`
    pub const WAP_SL: crate::Name = crate::Name::new_unchecked("vnd.wap.sl");
    /// `vnd.wap.slc`
    pub const WAP_SLC: crate::Name = crate::Name::new_unchecked("vnd.wap.slc");
    /// `vnd.wap.wbmp`
    pub const WAP_WBMP: crate::Name = crate::Name::new_unchecked("vnd.wap.wbmp");
    /// `vnd.wap.wbxml`
    pub const WAP_WBXML: crate::Name = crate::Name::new_unchecked("vnd.wap.wbxml");
    /// `vnd.wap.wml`
    pub const WAP_WML: crate::Name = crate::Name::new_unchecked("vnd.wap.wml");
    /// `vnd.wap.wmlc`
    pub const WAP_WMLC: crate::Name = crate::Name::new_unchecked("vnd.wap.wmlc");
    /// `vnd.wap.wmlscript`
    pub const WAP_WMLSCRIPT: crate::Name = crate::Name::new_unchecked("vnd.wap.wmlscript");
    /// `vnd.wap.wmlscriptc`
    pub const WAP_WMLSCRIPTC: crate::Name = crate::Name::new_unchecked("vnd.wap.wmlscriptc");
    /// `vnd.webturbo`
    pub const WEBTURBO: crate::Name = crate::Name::new_unchecked("vnd.webturbo");
    /// `vnd.wfa.dpp`
    pub const WFA_DPP: crate::Name = crate::Name::new_unchecked("vnd.wfa.dpp");
    /// `vnd.wfa.p2p`
    pub const WFA_P2P: crate::Name = crate::Name::new_unchecked("vnd.wfa.p2p");
    /// `vnd.wfa.wsc`
    pub const WFA_WSC: crate::Name = crate::Name::new_unchecked("vnd.wfa.wsc");
    /// `vnd.windows.devicepairing`
    pub const WINDOWS_DEVICEPAIRING: crate::Name = crate::Name::new_unchecked("vnd.windows.devicepairing");
    /// `vnd.wmc`
    pub const WMC: crate::Name = crate::Name::new_unchecked("vnd.wmc");
    /// `vnd.wmf.bootstrap`
    pub const WMF_BOOTSTRAP: crate::Name = crate::Name::new_unchecked("vnd.wmf.bootstrap");
    /// `vnd.wolfram.mathematica`
    pub const WOLFRAM_MATHEMATICA: crate::Name = crate::Name::new_unchecked("vnd.wolfram.mathematica");
    /// `vnd.wolfram.mathematica.package`
    pub const WOLFRAM_MATHEMATICA_PACKAGE: crate::Name = crate::Name::new_unchecked("vnd.wolfram.mathematica.package");
    /// `vnd.wolfram.player`
    pub const WOLFRAM_PLAYER: crate::Name = crate::Name::new_unchecked("vnd.wolfram.player");
    /// `vnd.wordperfect`
    pub const WORDPERFECT: crate::Name = crate::Name::new_unchecked("vnd.wordperfect");
    /// `vnd.wqd`
    pub const WQD: crate::Name = crate::Name::new_unchecked("vnd.wqd");
    /// `vnd.wrq-hp3000-labelled`
    pub const WRQ_HP3000_LABELLED: crate::Name = crate::Name::new_unchecked("vnd.wrq-hp3000-labelled");
    /// `vnd.wt.stf`
    pub const WT_STF: crate::Name = crate::Name::new_unchecked("vnd.wt.stf");
    /// `vnd.wv.csp`
    pub const WV_CSP: crate::Name = crate::Name::new_unchecked("vnd.wv.csp");
    /// `vnd.wv.ssp`
    pub const WV_SSP: crate::Name = crate::Name::new_unchecked("vnd.wv.ssp");
    /// `vnd.xacml`
    pub const XACML: crate::Name = crate::Name::new_unchecked("vnd.xacml");
    /// `vnd.xara`
    pub const XARA: crate::Name = crate::Name::new_unchecked("vnd.xara");
    /// `vnd.xfdl`
    pub const XFDL: crate::Name = crate::Name::new_unchecked("vnd.xfdl");
    /// `vnd.xfdl.webform`
    pub const XFDL_WEBFORM: crate::Name = crate::Name::new_unchecked("vnd.xfdl.webform");
    /// `vnd.xiff`
    pub const XIFF: crate::Name = crate::Name::new_unchecked("vnd.xiff");
    /// `vnd.xmi`
    pub const XMI: crate::Name = crate::Name::new_unchecked("vnd.xmi");
    /// `vnd.xmpie.cpkg`
    pub const XMPIE_CPKG: crate::Name = crate::Name::new_unchecked("vnd.xmpie.cpkg");
    /// `vnd.xmpie.dpkg`
    pub const XMPIE_DPKG: crate::Name = crate::Name::new_unchecked("vnd.xmpie.dpkg");
    /// `vnd.xmpie.plan`
    pub const XMPIE_PLAN: crate::Name = crate::Name::new_unchecked("vnd.xmpie.plan");
    /// `vnd.xmpie.ppkg`
    pub const XMPIE_PPKG: crate::Name = crate::Name::new_unchecked("vnd.xmpie.ppkg");
    /// `vnd.xmpie.xlim`
    pub const XMPIE_XLIM: crate::Name = crate::Name::new_unchecked("vnd.xmpie.xlim");
    /// `vnd.yamaha.hv-dic`
    pub const YAMAHA_HV_DIC: crate::Name = crate::Name::new_unchecked("vnd.yamaha.hv-dic");
    /// `vnd.yamaha.hv-script`
    pub const YAMAHA_HV_SCRIPT: crate::Name = crate::Name::new_unchecked("vnd.yamaha.hv-script");
    /// `vnd.yamaha.hv-voice`
    pub const YAMAHA_HV_VOICE: crate::Name = crate::Name::new_unchecked("vnd.yamaha.hv-voice");
    /// `vnd.yamaha.openscoreformat`
    pub const YAMAHA_OPENSCOREFORMAT: crate::Name = crate::Name::new_unchecked("vnd.yamaha.openscoreformat");
    /// `vnd.yamaha.openscoreformat.osfpvg`
    pub const YAMAHA_OPENSCOREFORMAT_OSFPVG: crate::Name = crate::Name::new_unchecked("vnd.yamaha.openscoreformat.osfpvg");
    /// `vnd.yamaha.remote-setup`
    pub const YAMAHA_REMOTE_SETUP: crate::Name = crate::Name::new_unchecked("vnd.yamaha.remote-setup");
    /// `vnd.yamaha.smaf-audio`
    pub const YAMAHA_SMAF_AUDIO: crate::Name = crate::Name::new_unchecked("vnd.yamaha.smaf-audio");
    /// `vnd.yamaha.smaf-phrase`
    pub const YAMAHA_SMAF_PHRASE: crate::Name = crate::Name::new_unchecked("vnd.yamaha.smaf-phrase");
    /// `vnd.yamaha.through-ngn`
    pub const YAMAHA_THROUGH_NGN: crate::Name = crate::Name::new_unchecked("vnd.yamaha.through-ngn");
    /// `vnd.yamaha.tunnel-udpencap`
    pub const YAMAHA_TUNNEL_UDPENCAP: crate::Name = crate::Name::new_unchecked("vnd.yamaha.tunnel-udpencap");
    /// `vnd.yaoweme`
    pub const YAOWEME: crate::Name = crate::Name::new_unchecked("vnd.yaoweme");
    /// `vnd.yellowriver-custom-menu`
    pub const YELLOWRIVER_CUSTOM_MENU: crate::Name = crate::Name::new_unchecked("vnd.yellowriver-custom-menu");
    /// `vnd.youtube.yt`
    pub const YOUTUBE_YT: crate::Name = crate::Name::new_unchecked("vnd.youtube.yt");
    /// `vnd.zbrush.pcx`
    pub const ZBRUSH_PCX: crate::Name = crate::Name::new_unchecked("vnd.zbrush.pcx");
    /// `vnd.zul`
    pub const ZUL: crate::Name = crate::Name::new_unchecked("vnd.zul");
    /// `vnd.zzazz.deck`
    pub const ZZAZZ_DECK: crate::Name = crate::Name::new_unchecked("vnd.zzazz.deck");
}

/// Unregistered subtypes starting with `x-`.
pub mod x_ {
    /// `x-7z-compressed`
    pub const _7Z_COMPRESSED: crate::Name = crate::Name::new_unchecked("x-7z-compressed");
    /// `x-abiword`
    pub const ABIWORD: crate::Name = crate::Name::new_unchecked("x-abiword");
    /// `x-bzip`
    pub const BZIP: crate::Name = crate::Name::new_unchecked("x-bzip");
    /// `x-bzip2`
    pub const BZIP2: crate::Name = crate::Name::new_unchecked("x-bzip2");
    /// `x-cdf`
    pub const CDF: crate::Name = crate::Name::new_unchecked("x-cdf");
    /// `x-csh`
    pub const CSH: crate::Name = crate::Name::new_unchecked("x-csh");
    /// `x-ecmascript`
    pub const ECMASCRIPT: crate::Name = crate::Name::new_unchecked("x-ecmascript");
    /// `x-emf`
    pub const EMF: crate::Name = crate::Name::new_unchecked("x-emf");
    /// `x-freearc`
    pub const FREEARC: crate::Name = crate::Name::new_unchecked("x-freearc");
    /// `x-httpd-php`
    pub const HTTPD_PHP: crate::Name = crate::Name::new_unchecked("x-httpd-php");
    /// `x-icon`
    pub const ICON: crate::Name = crate::Name::new_unchecked("x-icon");
    /// `x-javascript`
    pub const JAVASCRIPT: crate::Name = crate::Name::new_unchecked("x-javascript");
    /// `x-midi`
    pub const MIDI: crate::Name = crate::Name::new_unchecked("x-midi");
    /// `x-mixed-replace`
    pub const MIXED_REPLACE: crate::Name = crate::Name::new_unchecked("x-mixed-replace");
    /// `x-msgpack`
    pub const MSGPACK: crate::Name = crate::Name::new_unchecked("x-msgpack");
    /// `x-msvideo`
    pub const MSVIDEO: crate::Name = crate::Name::new_unchecked("x-msvideo");
    /// `x-pki-message`
    pub const PKI_MESSAGE: crate::Name = crate::Name::new_unchecked("x-pki-message");
    /// `x-sh`
    pub const SH: crate::Name = crate::Name::new_unchecked("x-sh");
    /// `x-shockwave-flash`
    pub const SHOCKWAVE_FLASH: crate::Name = crate::Name::new_unchecked("x-shockwave-flash");
    /// `x-tar`
    pub const TAR: crate::Name = crate::Name::new_unchecked("x-tar");
    /// `x-wmf`
    pub const WMF: crate::Name = crate::Name::new_unchecked("x-wmf");
    /// `x-www-form-urlencoded`
    pub const WWW_FORM_URLENCODED: crate::Name = crate::Name::new_unchecked("x-www-form-urlencoded");
    /// `x-x509-ca-cert`
    pub const X509_CA_CERT: crate::Name = crate::Name::new_unchecked("x-x509-ca-cert");
    /// `x-x509-ca-ra-cert`
    pub const X509_CA_RA_CERT: crate::Name = crate::Name::new_unchecked("x-x509-ca-ra-cert");
    /// `x-x509-next-ca-cert`
    pub const X509_NEXT_CA_CERT: crate::Name = crate::Name::new_unchecked("x-x509-next-ca-cert");
    /// `x-yaml`
    pub const YAML: crate::Name = crate::Name::new_unchecked("x-yaml");
}

/// `*`
pub const _STAR: crate::Name = crate::Name::new_unchecked("*");
/// `1d-interleaved-parityfec`
pub const _1D_INTERLEAVED_PARITYFEC: crate::Name = crate::Name::new_unchecked("1d-interleaved-parityfec");
/// `32kadpcm`
pub const _32KADPCM: crate::Name = crate::Name::new_unchecked("32kadpcm");
/// `3gpdash-qoe-report`
pub const _3GPDASH_QOE_REPORT: crate::Name = crate::Name::new_unchecked("3gpdash-qoe-report");
/// `3gpp`
pub const _3GPP: crate::Name = crate::Name::new_unchecked("3gpp");
/// `3gpp-ims`
pub const _3GPP_IMS: crate::Name = crate::Name::new_unchecked("3gpp-ims");
/// `3gpp-tt`
pub const _3GPP_TT: crate::Name = crate::Name::new_unchecked("3gpp-tt");
/// `3gpp2`
pub const _3GPP2: crate::Name = crate::Name::new_unchecked("3gpp2");
/// `3gppHal`
pub const _3GPP_HAL: crate::Name = crate::Name::new_unchecked("3gppHal");
/// `3gppHalForms`
pub const _3GPP_HAL_FORMS: crate::Name = crate::Name::new_unchecked("3gppHalForms");
/// `3mf`
pub const _3MF: crate::Name = crate::Name::new_unchecked("3mf");
/// `A2L`
pub const A2L: crate::Name = crate::Name::new_unchecked("A2L");
/// `aac`
pub const AAC: crate::Name = crate::Name::new_unchecked("aac");
/// `ac3`
pub const AC3: crate::Name = crate::Name::new_unchecked("ac3");
/// `ace`
pub const ACE: crate::Name = crate::Name::new_unchecked("ace");
/// `aces`
pub const ACES: crate::Name = crate::Name::new_unchecked("aces");
/// `activemessage`
pub const ACTIVEMESSAGE: crate::Name = crate::Name::new_unchecked("activemessage");
/// `activity`
pub const ACTIVITY: crate::Name = crate::Name::new_unchecked("activity");
/// `alternative`
pub const ALTERNATIVE: crate::Name = crate::Name::new_unchecked("alternative");
/// `alto-costmap`
pub const ALTO_COSTMAP: crate::Name = crate::Name::new_unchecked("alto-costmap");
/// `alto-costmapfilter`
pub const ALTO_COSTMAPFILTER: crate::Name = crate::Name::new_unchecked("alto-costmapfilter");
/// `alto-directory`
pub const ALTO_DIRECTORY: crate::Name = crate::Name::new_unchecked("alto-directory");
/// `alto-endpointcost`
pub const ALTO_ENDPOINTCOST: crate::Name = crate::Name::new_unchecked("alto-endpointcost");
/// `alto-endpointcostparams`
pub const ALTO_ENDPOINTCOSTPARAMS: crate::Name = crate::Name::new_unchecked("alto-endpointcostparams");
/// `alto-endpointprop`
pub const ALTO_ENDPOINTPROP: crate::Name = crate::Name::new_unchecked("alto-endpointprop");
/// `alto-endpointpropparams`
pub const ALTO_ENDPOINTPROPPARAMS: crate::Name = crate::Name::new_unchecked("alto-endpointpropparams");
/// `alto-error`
pub const ALTO_ERROR: crate::Name = crate::Name::new_unchecked("alto-error");
/// `alto-networkmap`
pub const ALTO_NETWORKMAP: crate::Name = crate::Name::new_unchecked("alto-networkmap");
/// `alto-networkmapfilter`
pub const ALTO_NETWORKMAPFILTER: crate::Name = crate::Name::new_unchecked("alto-networkmapfilter");
/// `alto-updatestreamcontrol`
pub const ALTO_UPDATESTREAMCONTROL: crate::Name = crate::Name::new_unchecked("alto-updatestreamcontrol");
/// `alto-updatestreamparams`
pub const ALTO_UPDATESTREAMPARAMS: crate::Name = crate::Name::new_unchecked("alto-updatestreamparams");
/// `AML`
pub const AML: crate::Name = crate::Name::new_unchecked("AML");
/// `AMR`
pub const AMR: crate::Name = crate::Name::new_unchecked("AMR");
/// `AMR-WB`
pub const AMR_WB: crate::Name = crate::Name::new_unchecked("AMR-WB");
/// `amr-wb+`
pub const AMR_WB_PLUS: crate::Name = crate::Name::new_unchecked("amr-wb+");
/// `andrew-inset`
pub const ANDREW_INSET: crate::Name = crate::Name::new_unchecked("andrew-inset");
/// `appledouble`
pub const APPLEDOUBLE: crate::Name = crate::Name::new_unchecked("appledouble");
/// `applefile`
pub const APPLEFILE: crate::Name = crate::Name::new_unchecked("applefile");
/// `application`
pub const APPLICATION: crate::Name = crate::Name::new_unchecked("application");
/// `aptx`
pub const APTX: crate::Name = crate::Name::new_unchecked("aptx");
/// `asc`
pub const ASC: crate::Name = crate::Name::new_unchecked("asc");
/// `at`
pub const AT: crate::Name = crate::Name::new_unchecked("at");
/// `ATF`
pub const ATF: crate::Name = crate::Name::new_unchecked("ATF");
/// `ATFX`
pub const ATFX: crate::Name = crate::Name::new_unchecked("ATFX");
/// `atom`
pub const ATOM: crate::Name = crate::Name::new_unchecked("atom");
/// `atomcat`
pub const ATOMCAT: crate::Name = crate::Name::new_unchecked("atomcat");
/// `atomdeleted`
pub const ATOMDELETED: crate::Name = crate::Name::new_unchecked("atomdeleted");
/// `atomicmail`
pub const ATOMICMAIL: crate::Name = crate::Name::new_unchecked("atomicmail");
/// `atomsvc`
pub const ATOMSVC: crate::Name = crate::Name::new_unchecked("atomsvc");
/// `ATRAC-ADVANCED-LOSSLESS`
pub const ATRAC_ADVANCED_LOSSLESS: crate::Name = crate::Name::new_unchecked("ATRAC-ADVANCED-LOSSLESS");
/// `ATRAC-X`
pub const ATRAC_X: crate::Name = crate::Name::new_unchecked("ATRAC-X");
/// `ATRAC3`
pub const ATRAC3: crate::Name = crate::Name::new_unchecked("ATRAC3");
/// `atsc-dwd`
pub const ATSC_DWD: crate::Name = crate::Name::new_unchecked("atsc-dwd");
/// `atsc-dynamic-event-message`
pub const ATSC_DYNAMIC_EVENT_MESSAGE: crate::Name = crate::Name::new_unchecked("atsc-dynamic-event-message");
/// `atsc-held`
pub const ATSC_HELD: crate::Name = crate::Name::new_unchecked("atsc-held");
/// `atsc-rdt`
pub const ATSC_RDT: crate::Name = crate::Name::new_unchecked("atsc-rdt");
/// `atsc-rsat`
pub const ATSC_RSAT: crate::Name = crate::Name::new_unchecked("atsc-rsat");
/// `ATXML`
pub const ATXML: crate::Name = crate::Name::new_unchecked("ATXML");
/// `audio`
pub const AUDIO: crate::Name = crate::Name::new_unchecked("audio");
/// `auth-policy`
pub const AUTH_POLICY: crate::Name = crate::Name::new_unchecked("auth-policy");
/// `AV1`
pub const AV1: crate::Name = crate::Name::new_unchecked("AV1");
/// `avci`
pub const AVCI: crate::Name = crate::Name::new_unchecked("avci");
/// `avcs`
pub const AVCS: crate::Name = crate::Name::new_unchecked("avcs");
/// `avif`
pub const AVIF: crate::Name = crate::Name::new_unchecked("avif");
/// `bacnet-xdd`
pub const BACNET_XDD: crate::Name = crate::Name::new_unchecked("bacnet-xdd");
/// `basic`
pub const BASIC: crate::Name = crate::Name::new_unchecked("basic");
/// `batch-SMTP`
pub const BATCH_SMTP: crate::Name = crate::Name::new_unchecked("batch-SMTP");
/// `beep`
pub const BEEP: crate::Name = crate::Name::new_unchecked("beep");
/// `ber`
pub const BER: crate::Name = crate::Name::new_unchecked("ber");
/// `bmp`
pub const BMP: crate::Name = crate::Name::new_unchecked("bmp");
/// `BMPEG`
pub const BMPEG: crate::Name = crate::Name::new_unchecked("BMPEG");
/// `boundary`
pub const BOUNDARY: crate::Name = crate::Name::new_unchecked("boundary");
/// `BT656`
pub const BT656: crate::Name = crate::Name::new_unchecked("BT656");
/// `BV16`
pub const BV16: crate::Name = crate::Name::new_unchecked("BV16");
/// `BV32`
pub const BV32: crate::Name = crate::Name::new_unchecked("BV32");
/// `byteranges`
pub const BYTERANGES: crate::Name = crate::Name::new_unchecked("byteranges");
/// `cache-manifest`
pub const CACHE_MANIFEST: crate::Name = crate::Name::new_unchecked("cache-manifest");
/// `calendar`
pub const CALENDAR: crate::Name = crate::Name::new_unchecked("calendar");
/// `call-completion`
pub const CALL_COMPLETION: crate::Name = crate::Name::new_unchecked("call-completion");
/// `CALS-1840`
pub const CALS_1840: crate::Name = crate::Name::new_unchecked("CALS-1840");
/// `captive`
pub const CAPTIVE: crate::Name = crate::Name::new_unchecked("captive");
/// `cbor`
pub const CBOR: crate::Name = crate::Name::new_unchecked("cbor");
/// `cbor-seq`
pub const CBOR_SEQ: crate::Name = crate::Name::new_unchecked("cbor-seq");
/// `cccex`
pub const CCCEX: crate::Name = crate::Name::new_unchecked("cccex");
/// `ccmp`
pub const CCMP: crate::Name = crate::Name::new_unchecked("ccmp");
/// `ccxml`
pub const CCXML: crate::Name = crate::Name::new_unchecked("ccxml");
/// `CDFX`
pub const CDFX: crate::Name = crate::Name::new_unchecked("CDFX");
/// `cdmi-capability`
pub const CDMI_CAPABILITY: crate::Name = crate::Name::new_unchecked("cdmi-capability");
/// `cdmi-container`
pub const CDMI_CONTAINER: crate::Name = crate::Name::new_unchecked("cdmi-container");
/// `cdmi-domain`
pub const CDMI_DOMAIN: crate::Name = crate::Name::new_unchecked("cdmi-domain");
/// `cdmi-object`
pub const CDMI_OBJECT: crate::Name = crate::Name::new_unchecked("cdmi-object");
/// `cdmi-queue`
pub const CDMI_QUEUE: crate::Name = crate::Name::new_unchecked("cdmi-queue");
/// `cdni`
pub const CDNI: crate::Name = crate::Name::new_unchecked("cdni");
/// `CEA`
pub const CEA: crate::Name = crate::Name::new_unchecked("CEA");
/// `cea-2018`
pub const CEA_2018: crate::Name = crate::Name::new_unchecked("cea-2018");
/// `CelB`
pub const CEL_B: crate::Name = crate::Name::new_unchecked("CelB");
/// `cellml`
pub const CELLML: crate::Name = crate::Name::new_unchecked("cellml");
/// `cfw`
pub const CFW: crate::Name = crate::Name::new_unchecked("cfw");
/// `cgm`
pub const CGM: crate::Name = crate::Name::new_unchecked("cgm");
/// `charset`
pub const CHARSET: crate::Name = crate::Name::new_unchecked("charset");
/// `chemical`
pub const CHEMICAL: crate::Name = crate::Name::new_unchecked("chemical");
/// `city`
pub const CITY: crate::Name = crate::Name::new_unchecked("city");
/// `clearmode`
pub const CLEARMODE: crate::Name = crate::Name::new_unchecked("clearmode");
/// `clr`
pub const CLR: crate::Name = crate::Name::new_unchecked("clr");
/// `clue`
pub const CLUE: crate::Name = crate::Name::new_unchecked("clue");
/// `clue_info`
pub const CLUE_INFO: crate::Name = crate::Name::new_unchecked("clue_info");
/// `cms`
pub const CMS: crate::Name = crate::Name::new_unchecked("cms");
/// `CN`
pub const CN: crate::Name = crate::Name::new_unchecked("CN");
/// `cnrp`
pub const CNRP: crate::Name = crate::Name::new_unchecked("cnrp");
/// `coap-group`
pub const COAP_GROUP: crate::Name = crate::Name::new_unchecked("coap-group");
/// `coap-payload`
pub const COAP_PAYLOAD: crate::Name = crate::Name::new_unchecked("coap-payload");
/// `collection`
pub const COLLECTION: crate::Name = crate::Name::new_unchecked("collection");
/// `commonground`
pub const COMMONGROUND: crate::Name = crate::Name::new_unchecked("commonground");
/// `conference-info`
pub const CONFERENCE_INFO: crate::Name = crate::Name::new_unchecked("conference-info");
/// `cose`
pub const COSE: crate::Name = crate::Name::new_unchecked("cose");
/// `cose-key`
pub const COSE_KEY: crate::Name = crate::Name::new_unchecked("cose-key");
/// `cose-key-set`
pub const COSE_KEY_SET: crate::Name = crate::Name::new_unchecked("cose-key-set");
/// `cpl`
pub const CPL: crate::Name = crate::Name::new_unchecked("cpl");
/// `cql`
pub const CQL: crate::Name = crate::Name::new_unchecked("cql");
/// `cql-expression`
pub const CQL_EXPRESSION: crate::Name = crate::Name::new_unchecked("cql-expression");
/// `cql-identifier`
pub const CQL_IDENTIFIER: crate::Name = crate::Name::new_unchecked("cql-identifier");
/// `csrattrs`
pub const CSRATTRS: crate::Name = crate::Name::new_unchecked("csrattrs");
/// `css`
pub const CSS: crate::Name = crate::Name::new_unchecked("css");
/// `csta`
pub const CSTA: crate::Name = crate::Name::new_unchecked("csta");
/// `CSTAdata`
pub const CSTADATA: crate::Name = crate::Name::new_unchecked("CSTAdata");
/// `csv`
pub const CSV: crate::Name = crate::Name::new_unchecked("csv");
/// `csv-schema`
pub const CSV_SCHEMA: crate::Name = crate::Name::new_unchecked("csv-schema");
/// `csvm`
pub const CSVM: crate::Name = crate::Name::new_unchecked("csvm");
/// `cwt`
pub const CWT: crate::Name = crate::Name::new_unchecked("cwt");
/// `cybercash`
pub const CYBERCASH: crate::Name = crate::Name::new_unchecked("cybercash");
/// `dash`
pub const DASH: crate::Name = crate::Name::new_unchecked("dash");
/// `dash-patch`
pub const DASH_PATCH: crate::Name = crate::Name::new_unchecked("dash-patch");
/// `dashdelta`
pub const DASHDELTA: crate::Name = crate::Name::new_unchecked("dashdelta");
/// `DAT12`
pub const DAT12: crate::Name = crate::Name::new_unchecked("DAT12");
/// `davmount`
pub const DAVMOUNT: crate::Name = crate::Name::new_unchecked("davmount");
/// `dca-rft`
pub const DCA_RFT: crate::Name = crate::Name::new_unchecked("dca-rft");
/// `DCD`
pub const DCD: crate::Name = crate::Name::new_unchecked("DCD");
/// `dec-dx`
pub const DEC_DX: crate::Name = crate::Name::new_unchecked("dec-dx");
/// `delsp`
pub const DELSP: crate::Name = crate::Name::new_unchecked("delsp");
/// `der`
pub const DER: crate::Name = crate::Name::new_unchecked("der");
/// `dialog-info`
pub const DIALOG_INFO: crate::Name = crate::Name::new_unchecked("dialog-info");
/// `dicom`
pub const DICOM: crate::Name = crate::Name::new_unchecked("dicom");
/// `dicom-rle`
pub const DICOM_RLE: crate::Name = crate::Name::new_unchecked("dicom-rle");
/// `digest`
pub const DIGEST: crate::Name = crate::Name::new_unchecked("digest");
/// `DII`
pub const DII: crate::Name = crate::Name::new_unchecked("DII");
/// `directory`
pub const DIRECTORY: crate::Name = crate::Name::new_unchecked("directory");
/// `DIT`
pub const DIT: crate::Name = crate::Name::new_unchecked("DIT");
/// `dls`
pub const DLS: crate::Name = crate::Name::new_unchecked("dls");
/// `dns`
pub const DNS: crate::Name = crate::Name::new_unchecked("dns");
/// `dns-message`
pub const DNS_MESSAGE: crate::Name = crate::Name::new_unchecked("dns-message");
/// `dots`
pub const DOTS: crate::Name = crate::Name::new_unchecked("dots");
/// `dskpp`
pub const DSKPP: crate::Name = crate::Name::new_unchecked("dskpp");
/// `dsr-es201108`
pub const DSR_ES201108: crate::Name = crate::Name::new_unchecked("dsr-es201108");
/// `dsr-es202050`
pub const DSR_ES202050: crate::Name = crate::Name::new_unchecked("dsr-es202050");
/// `dsr-es202211`
pub const DSR_ES202211: crate::Name = crate::Name::new_unchecked("dsr-es202211");
/// `dsr-es202212`
pub const DSR_ES202212: crate::Name = crate::Name::new_unchecked("dsr-es202212");
/// `dssc`
pub const DSSC: crate::Name = crate::Name::new_unchecked("dssc");
/// `DV`
pub const DV: crate::Name = crate::Name::new_unchecked("DV");
/// `dvcs`
pub const DVCS: crate::Name = crate::Name::new_unchecked("dvcs");
/// `DVI4`
pub const DVI4: crate::Name = crate::Name::new_unchecked("DVI4");
/// `e57`
pub const E57: crate::Name = crate::Name::new_unchecked("e57");
/// `eac3`
pub const EAC3: crate::Name = crate::Name::new_unchecked("eac3");
/// `ecmascript`
pub const ECMASCRIPT: crate::Name = crate::Name::new_unchecked("ecmascript");
/// `EDI-consent`
pub const EDI_CONSENT: crate::Name = crate::Name::new_unchecked("EDI-consent");
/// `EDI-X12`
pub const EDI_X12: crate::Name = crate::Name::new_unchecked("EDI-X12");
/// `EDIFACT`
pub const EDIFACT: crate::Name = crate::Name::new_unchecked("EDIFACT");
/// `efi`
pub const EFI: crate::Name = crate::Name::new_unchecked("efi");
/// `elm`
pub const ELM: crate::Name = crate::Name::new_unchecked("elm");
/// `EmergencyCallData.cap`
pub const EMERGENCY_CALL_DATA_CAP: crate::Name = crate::Name::new_unchecked("EmergencyCallData.cap");
/// `EmergencyCallData.Comment`
pub const EMERGENCY_CALL_DATA_COMMENT: crate::Name = crate::Name::new_unchecked("EmergencyCallData.Comment");
/// `EmergencyCallData.Control`
pub const EMERGENCY_CALL_DATA_CONTROL: crate::Name = crate::Name::new_unchecked("EmergencyCallData.Control");
/// `EmergencyCallData.DeviceInfo`
pub const EMERGENCY_CALL_DATA_DEVICE_INFO: crate::Name = crate::Name::new_unchecked("EmergencyCallData.DeviceInfo");
/// `EmergencyCallData.eCall.MSD`
pub const EMERGENCY_CALL_DATA_E_CALL_MSD: crate::Name = crate::Name::new_unchecked("EmergencyCallData.eCall.MSD");
/// `EmergencyCallData.ProviderInfo`
pub const EMERGENCY_CALL_DATA_PROVIDER_INFO: crate::Name = crate::Name::new_unchecked("EmergencyCallData.ProviderInfo");
/// `EmergencyCallData.ServiceInfo`
pub const EMERGENCY_CALL_DATA_SERVICE_INFO: crate::Name = crate::Name::new_unchecked("EmergencyCallData.ServiceInfo");
/// `EmergencyCallData.SubscriberInfo`
pub const EMERGENCY_CALL_DATA_SUBSCRIBER_INFO: crate::Name = crate::Name::new_unchecked("EmergencyCallData.SubscriberInfo");
/// `EmergencyCallData.VEDS`
pub const EMERGENCY_CALL_DATA_VEDS: crate::Name = crate::Name::new_unchecked("EmergencyCallData.VEDS");
/// `emf`
pub const EMF: crate::Name = crate::Name::new_unchecked("emf");
/// `emma`
pub const EMMA: crate::Name = crate::Name::new_unchecked("emma");
/// `emotionml`
pub const EMOTIONML: crate::Name = crate::Name::new_unchecked("emotionml");
/// `encaprtp`
pub const ENCAPRTP: crate::Name = crate::Name::new_unchecked("encaprtp");
/// `encrypted`
pub const ENCRYPTED: crate::Name = crate::Name::new_unchecked("encrypted");
/// `enriched`
pub const ENRICHED: crate::Name = crate::Name::new_unchecked("enriched");
/// `epp`
pub const EPP: crate::Name = crate::Name::new_unchecked("epp");
/// `epub`
pub const EPUB: crate::Name = crate::Name::new_unchecked("epub");
/// `eshop`
pub const ESHOP: crate::Name = crate::Name::new_unchecked("eshop");
/// `event-stream`
pub const EVENT_STREAM: crate::Name = crate::Name::new_unchecked("event-stream");
/// `EVRC`
pub const EVRC: crate::Name = crate::Name::new_unchecked("EVRC");
/// `EVRC-QCP`
pub const EVRC_QCP: crate::Name = crate::Name::new_unchecked("EVRC-QCP");
/// `EVRC0`
pub const EVRC0: crate::Name = crate::Name::new_unchecked("EVRC0");
/// `EVRC1`
pub const EVRC1: crate::Name = crate::Name::new_unchecked("EVRC1");
/// `EVRCB`
pub const EVRCB: crate::Name = crate::Name::new_unchecked("EVRCB");
/// `EVRCB0`
pub const EVRCB0: crate::Name = crate::Name::new_unchecked("EVRCB0");
/// `EVRCB1`
pub const EVRCB1: crate::Name = crate::Name::new_unchecked("EVRCB1");
/// `EVRCNW`
pub const EVRCNW: crate::Name = crate::Name::new_unchecked("EVRCNW");
/// `EVRCNW0`
pub const EVRCNW0: crate::Name = crate::Name::new_unchecked("EVRCNW0");
/// `EVRCNW1`
pub const EVRCNW1: crate::Name = crate::Name::new_unchecked("EVRCNW1");
/// `EVRCWB`
pub const EVRCWB: crate::Name = crate::Name::new_unchecked("EVRCWB");
/// `EVRCWB0`
pub const EVRCWB0: crate::Name = crate::Name::new_unchecked("EVRCWB0");
/// `EVRCWB1`
pub const EVRCWB1: crate::Name = crate::Name::new_unchecked("EVRCWB1");
/// `EVS`
pub const EVS: crate::Name = crate::Name::new_unchecked("EVS");
/// `example`
pub const EXAMPLE: crate::Name = crate::Name::new_unchecked("example");
/// `exi`
pub const EXI: crate::Name = crate::Name::new_unchecked("exi");
/// `expect-ct-report`
pub const EXPECT_CT_REPORT: crate::Name = crate::Name::new_unchecked("expect-ct-report");
/// `express`
pub const EXPRESS: crate::Name = crate::Name::new_unchecked("express");
/// `fastinfoset`
pub const FASTINFOSET: crate::Name = crate::Name::new_unchecked("fastinfoset");
/// `fastsoap`
pub const FASTSOAP: crate::Name = crate::Name::new_unchecked("fastsoap");
/// `fdt`
pub const FDT: crate::Name = crate::Name::new_unchecked("fdt");
/// `FFV1`
pub const FFV1: crate::Name = crate::Name::new_unchecked("FFV1");
/// `fhir`
pub const FHIR: crate::Name = crate::Name::new_unchecked("fhir");
/// `fhirpath`
pub const FHIRPATH: crate::Name = crate::Name::new_unchecked("fhirpath");
/// `fits`
pub const FITS: crate::Name = crate::Name::new_unchecked("fits");
/// `flexfec`
pub const FLEXFEC: crate::Name = crate::Name::new_unchecked("flexfec");
/// `font`
pub const FONT: crate::Name = crate::Name::new_unchecked("font");
/// `font-sfnt`
pub const FONT_SFNT: crate::Name = crate::Name::new_unchecked("font-sfnt");
/// `font-tdpfr`
pub const FONT_TDPFR: crate::Name = crate::Name::new_unchecked("font-tdpfr");
/// `font-woff`
pub const FONT_WOFF: crate::Name = crate::Name::new_unchecked("font-woff");
/// `form-data`
pub const FORM_DATA: crate::Name = crate::Name::new_unchecked("form-data");
/// `format`
pub const FORMAT: crate::Name = crate::Name::new_unchecked("format");
/// `framework-attributes`
pub const FRAMEWORK_ATTRIBUTES: crate::Name = crate::Name::new_unchecked("framework-attributes");
/// `fwdred`
pub const FWDRED: crate::Name = crate::Name::new_unchecked("fwdred");
/// `g3fax`
pub const G3FAX: crate::Name = crate::Name::new_unchecked("g3fax");
/// `G711-0`
pub const G711_0: crate::Name = crate::Name::new_unchecked("G711-0");
/// `G719`
pub const G719: crate::Name = crate::Name::new_unchecked("G719");
/// `G722`
pub const G722: crate::Name = crate::Name::new_unchecked("G722");
/// `G7221`
pub const G7221: crate::Name = crate::Name::new_unchecked("G7221");
/// `G723`
pub const G723: crate::Name = crate::Name::new_unchecked("G723");
/// `G726-16`
pub const G726_16: crate::Name = crate::Name::new_unchecked("G726-16");
/// `G726-24`
pub const G726_24: crate::Name = crate::Name::new_unchecked("G726-24");
/// `G726-32`
pub const G726_32: crate::Name = crate::Name::new_unchecked("G726-32");
/// `G726-40`
pub const G726_40: crate::Name = crate::Name::new_unchecked("G726-40");
/// `G728`
pub const G728: crate::Name = crate::Name::new_unchecked("G728");
/// `G729`
pub const G729: crate::Name = crate::Name::new_unchecked("G729");
/// `G7291`
pub const G7291: crate::Name = crate::Name::new_unchecked("G7291");
/// `G729D`
pub const G729D: crate::Name = crate::Name::new_unchecked("G729D");
/// `G729E`
pub const G729E: crate::Name = crate::Name::new_unchecked("G729E");
/// `geo`
pub const GEO: crate::Name = crate::Name::new_unchecked("geo");
/// `geopackage`
pub const GEOPACKAGE: crate::Name = crate::Name::new_unchecked("geopackage");
/// `geoxacml`
pub const GEOXACML: crate::Name = crate::Name::new_unchecked("geoxacml");
/// `gff3`
pub const GFF3: crate::Name = crate::Name::new_unchecked("gff3");
/// `gif`
pub const GIF: crate::Name = crate::Name::new_unchecked("gif");
/// `gltf`
pub const GLTF: crate::Name = crate::Name::new_unchecked("gltf");
/// `gltf-binary`
pub const GLTF_BINARY: crate::Name = crate::Name::new_unchecked("gltf-binary");
/// `gltf-buffer`
pub const GLTF_BUFFER: crate::Name = crate::Name::new_unchecked("gltf-buffer");
/// `gml`
pub const GML: crate::Name = crate::Name::new_unchecked("gml");
/// `grammar-ref-list`
pub const GRAMMAR_REF_LIST: crate::Name = crate::Name::new_unchecked("grammar-ref-list");
/// `GSM`
pub const GSM: crate::Name = crate::Name::new_unchecked("GSM");
/// `GSM-EFR`
pub const GSM_EFR: crate::Name = crate::Name::new_unchecked("GSM-EFR");
/// `GSM-HR-08`
pub const GSM_HR_08: crate::Name = crate::Name::new_unchecked("GSM-HR-08");
/// `gzip`
pub const GZIP: crate::Name = crate::Name::new_unchecked("gzip");
/// `H224`
pub const H224: crate::Name = crate::Name::new_unchecked("H224");
/// `H261`
pub const H261: crate::Name = crate::Name::new_unchecked("H261");
/// `H263`
pub const H263: crate::Name = crate::Name::new_unchecked("H263");
/// `H263-1998`
pub const H263_1998: crate::Name = crate::Name::new_unchecked("H263-1998");
/// `H263-2000`
pub const H263_2000: crate::Name = crate::Name::new_unchecked("H263-2000");
/// `H264`
pub const H264: crate::Name = crate::Name::new_unchecked("H264");
/// `H264-RCDO`
pub const H264_RCDO: crate::Name = crate::Name::new_unchecked("H264-RCDO");
/// `H264-SVC`
pub const H264_SVC: crate::Name = crate::Name::new_unchecked("H264-SVC");
/// `H265`
pub const H265: crate::Name = crate::Name::new_unchecked("H265");
/// `header-set`
pub const HEADER_SET: crate::Name = crate::Name::new_unchecked("header-set");
/// `heic`
pub const HEIC: crate::Name = crate::Name::new_unchecked("heic");
/// `heic-sequence`
pub const HEIC_SEQUENCE: crate::Name = crate::Name::new_unchecked("heic-sequence");
/// `heif`
pub const HEIF: crate::Name = crate::Name::new_unchecked("heif");
/// `heif-sequence`
pub const HEIF_SEQUENCE: crate::Name = crate::Name::new_unchecked("heif-sequence");
/// `hej2k`
pub const HEJ2K: crate::Name = crate::Name::new_unchecked("hej2k");
/// `held`
pub const HELD: crate::Name = crate::Name::new_unchecked("held");
/// `hsj2`
pub const HSJ2: crate::Name = crate::Name::new_unchecked("hsj2");
/// `html`
pub const HTML: crate::Name = crate::Name::new_unchecked("html");
/// `http`
pub const HTTP: crate::Name = crate::Name::new_unchecked("http");
/// `hyperstudio`
pub const HYPERSTUDIO: crate::Name = crate::Name::new_unchecked("hyperstudio");
/// `ibe-key-request`
pub const IBE_KEY_REQUEST: crate::Name = crate::Name::new_unchecked("ibe-key-request");
/// `ibe-pkg-reply`
pub const IBE_PKG_REPLY: crate::Name = crate::Name::new_unchecked("ibe-pkg-reply");
/// `ibe-pp-data`
pub const IBE_PP_DATA: crate::Name = crate::Name::new_unchecked("ibe-pp-data");
/// `ief`
pub const IEF: crate::Name = crate::Name::new_unchecked("ief");
/// `iges`
pub const IGES: crate::Name = crate::Name::new_unchecked("iges");
/// `iLBC`
pub const I_LBC: crate::Name = crate::Name::new_unchecked("iLBC");
/// `im-iscomposing`
pub const IM_ISCOMPOSING: crate::Name = crate::Name::new_unchecked("im-iscomposing");
/// `image`
pub const IMAGE: crate::Name = crate::Name::new_unchecked("image");
/// `index`
pub const INDEX: crate::Name = crate::Name::new_unchecked("index");
/// `index.cmd`
pub const INDEX_CMD: crate::Name = crate::Name::new_unchecked("index.cmd");
/// `index.obj`
pub const INDEX_OBJ: crate::Name = crate::Name::new_unchecked("index.obj");
/// `index.response`
pub const INDEX_RESPONSE: crate::Name = crate::Name::new_unchecked("index.response");
/// `index.vnd`
pub const INDEX_VND: crate::Name = crate::Name::new_unchecked("index.vnd");
/// `inkml`
pub const INKML: crate::Name = crate::Name::new_unchecked("inkml");
/// `IOTP`
pub const IOTP: crate::Name = crate::Name::new_unchecked("IOTP");
/// `ip-mr_v2.5`
pub const IP_MR_V2_5: crate::Name = crate::Name::new_unchecked("ip-mr_v2.5");
/// `ipfix`
pub const IPFIX: crate::Name = crate::Name::new_unchecked("ipfix");
/// `ipp`
pub const IPP: crate::Name = crate::Name::new_unchecked("ipp");
/// `iso.segment`
pub const ISO_SEGMENT: crate::Name = crate::Name::new_unchecked("iso.segment");
/// `ISUP`
pub const ISUP: crate::Name = crate::Name::new_unchecked("ISUP");
/// `its`
pub const ITS: crate::Name = crate::Name::new_unchecked("its");
/// `java-archive`
pub const JAVA_ARCHIVE: crate::Name = crate::Name::new_unchecked("java-archive");
/// `javascript`
pub const JAVASCRIPT: crate::Name = crate::Name::new_unchecked("javascript");
/// `javascript1.0`
pub const JAVASCRIPT1_0: crate::Name = crate::Name::new_unchecked("javascript1.0");
/// `javascript1.1`
pub const JAVASCRIPT1_1: crate::Name = crate::Name::new_unchecked("javascript1.1");
/// `javascript1.2`
pub const JAVASCRIPT1_2: crate::Name = crate::Name::new_unchecked("javascript1.2");
/// `javascript1.3`
pub const JAVASCRIPT1_3: crate::Name = crate::Name::new_unchecked("javascript1.3");
/// `javascript1.4`
pub const JAVASCRIPT1_4: crate::Name = crate::Name::new_unchecked("javascript1.4");
/// `javascript1.5`
pub const JAVASCRIPT1_5: crate::Name = crate::Name::new_unchecked("javascript1.5");
/// `jcr-cnd`
pub const JCR_CND: crate::Name = crate::Name::new_unchecked("jcr-cnd");
/// `jf2feed`
pub const JF2FEED: crate::Name = crate::Name::new_unchecked("jf2feed");
/// `jls`
pub const JLS: crate::Name = crate::Name::new_unchecked("jls");
/// `jose`
pub const JOSE: crate::Name = crate::Name::new_unchecked("jose");
/// `jp2`
pub const JP2: crate::Name = crate::Name::new_unchecked("jp2");
/// `jpeg`
pub const JPEG: crate::Name = crate::Name::new_unchecked("jpeg");
/// `jpeg2000`
pub const JPEG2000: crate::Name = crate::Name::new_unchecked("jpeg2000");
/// `jph`
pub const JPH: crate::Name = crate::Name::new_unchecked("jph");
/// `jphc`
pub const JPHC: crate::Name = crate::Name::new_unchecked("jphc");
/// `jpm`
pub const JPM: crate::Name = crate::Name::new_unchecked("jpm");
/// `jpx`
pub const JPX: crate::Name = crate::Name::new_unchecked("jpx");
/// `jrd`
pub const JRD: crate::Name = crate::Name::new_unchecked("jrd");
/// `jscalendar`
pub const JSCALENDAR: crate::Name = crate::Name::new_unchecked("jscalendar");
/// `jscript`
pub const JSCRIPT: crate::Name = crate::Name::new_unchecked("jscript");
/// `json`
pub const JSON: crate::Name = crate::Name::new_unchecked("json");
/// `json-patch`
pub const JSON_PATCH: crate::Name = crate::Name::new_unchecked("json-patch");
/// `json-seq`
pub const JSON_SEQ: crate::Name = crate::Name::new_unchecked("json-seq");
/// `jwk`
pub const JWK: crate::Name = crate::Name::new_unchecked("jwk");
/// `jwk-set`
pub const JWK_SET: crate::Name = crate::Name::new_unchecked("jwk-set");
/// `jwt`
pub const JWT: crate::Name = crate::Name::new_unchecked("jwt");
/// `jxr`
pub const JXR: crate::Name = crate::Name::new_unchecked("jxr");
/// `jxrA`
pub const JXR_A: crate::Name = crate::Name::new_unchecked("jxrA");
/// `jxrS`
pub const JXR_S: crate::Name = crate::Name::new_unchecked("jxrS");
/// `jxs`
pub const JXS: crate::Name = crate::Name::new_unchecked("jxs");
/// `jxsc`
pub const JXSC: crate::Name = crate::Name::new_unchecked("jxsc");
/// `jxsi`
pub const JXSI: crate::Name = crate::Name::new_unchecked("jxsi");
/// `jxss`
pub const JXSS: crate::Name = crate::Name::new_unchecked("jxss");
/// `jxsv`
pub const JXSV: crate::Name = crate::Name::new_unchecked("jxsv");
/// `kpml-request`
pub const KPML_REQUEST: crate::Name = crate::Name::new_unchecked("kpml-request");
/// `kpml-response`
pub const KPML_RESPONSE: crate::Name = crate::Name::new_unchecked("kpml-response");
/// `ktx`
pub const KTX: crate::Name = crate::Name::new_unchecked("ktx");
/// `ktx2`
pub const KTX2: crate::Name = crate::Name::new_unchecked("ktx2");
/// `L16`
pub const L16: crate::Name = crate::Name::new_unchecked("L16");
/// `L20`
pub const L20: crate::Name = crate::Name::new_unchecked("L20");
/// `L24`
pub const L24: crate::Name = crate::Name::new_unchecked("L24");
/// `L8`
pub const L8: crate::Name = crate::Name::new_unchecked("L8");
/// `ld`
pub const LD: crate::Name = crate::Name::new_unchecked("ld");
/// `lgr`
pub const LGR: crate::Name = crate::Name::new_unchecked("lgr");
/// `link-format`
pub const LINK_FORMAT: crate::Name = crate::Name::new_unchecked("link-format");
/// `livescript`
pub const LIVESCRIPT: crate::Name = crate::Name::new_unchecked("livescript");
/// `load-control`
pub const LOAD_CONTROL: crate::Name = crate::Name::new_unchecked("load-control");
/// `lost`
pub const LOST: crate::Name = crate::Name::new_unchecked("lost");
/// `lostsync`
pub const LOSTSYNC: crate::Name = crate::Name::new_unchecked("lostsync");
/// `LPC`
pub const LPC: crate::Name = crate::Name::new_unchecked("LPC");
/// `lpf`
pub const LPF: crate::Name = crate::Name::new_unchecked("lpf");
/// `LXF`
pub const LXF: crate::Name = crate::Name::new_unchecked("LXF");
/// `mac-binhex40`
pub const MAC_BINHEX40: crate::Name = crate::Name::new_unchecked("mac-binhex40");
/// `macwriteii`
pub const MACWRITEII: crate::Name = crate::Name::new_unchecked("macwriteii");
/// `mads`
pub const MADS: crate::Name = crate::Name::new_unchecked("mads");
/// `manifest`
pub const MANIFEST: crate::Name = crate::Name::new_unchecked("manifest");
/// `marc`
pub const MARC: crate::Name = crate::Name::new_unchecked("marc");
/// `marcxml`
pub const MARCXML: crate::Name = crate::Name::new_unchecked("marcxml");
/// `markdown`
pub const MARKDOWN: crate::Name = crate::Name::new_unchecked("markdown");
/// `mathematica`
pub const MATHEMATICA: crate::Name = crate::Name::new_unchecked("mathematica");
/// `mathml`
pub const MATHML: crate::Name = crate::Name::new_unchecked("mathml");
/// `mathml-content`
pub const MATHML_CONTENT: crate::Name = crate::Name::new_unchecked("mathml-content");
/// `mathml-presentation`
pub const MATHML_PRESENTATION: crate::Name = crate::Name::new_unchecked("mathml-presentation");
/// `mbms-associated-procedure-description`
pub const MBMS_ASSOCIATED_PROCEDURE_DESCRIPTION: crate::Name = crate::Name::new_unchecked("mbms-associated-procedure-description");
/// `mbms-deregister`
pub const MBMS_DEREGISTER: crate::Name = crate::Name::new_unchecked("mbms-deregister");
/// `mbms-envelope`
pub const MBMS_ENVELOPE: crate::Name = crate::Name::new_unchecked("mbms-envelope");
/// `mbms-msk`
pub const MBMS_MSK: crate::Name = crate::Name::new_unchecked("mbms-msk");
/// `mbms-msk-response`
pub const MBMS_MSK_RESPONSE: crate::Name = crate::Name::new_unchecked("mbms-msk-response");
/// `mbms-protection-description`
pub const MBMS_PROTECTION_DESCRIPTION: crate::Name = crate::Name::new_unchecked("mbms-protection-description");
/// `mbms-reception-report`
pub const MBMS_RECEPTION_REPORT: crate::Name = crate::Name::new_unchecked("mbms-reception-report");
/// `mbms-register`
pub const MBMS_REGISTER: crate::Name = crate::Name::new_unchecked("mbms-register");
/// `mbms-register-response`
pub const MBMS_REGISTER_RESPONSE: crate::Name = crate::Name::new_unchecked("mbms-register-response");
/// `mbms-schedule`
pub const MBMS_SCHEDULE: crate::Name = crate::Name::new_unchecked("mbms-schedule");
/// `mbms-user-service-description`
pub const MBMS_USER_SERVICE_DESCRIPTION: crate::Name = crate::Name::new_unchecked("mbms-user-service-description");
/// `mbox`
pub const MBOX: crate::Name = crate::Name::new_unchecked("mbox");
/// `media_control`
pub const MEDIA_CONTROL: crate::Name = crate::Name::new_unchecked("media_control");
/// `media-policy-dataset`
pub const MEDIA_POLICY_DATASET: crate::Name = crate::Name::new_unchecked("media-policy-dataset");
/// `mediaservercontrol`
pub const MEDIASERVERCONTROL: crate::Name = crate::Name::new_unchecked("mediaservercontrol");
/// `MELP`
pub const MELP: crate::Name = crate::Name::new_unchecked("MELP");
/// `MELP1200`
pub const MELP1200: crate::Name = crate::Name::new_unchecked("MELP1200");
/// `MELP2400`
pub const MELP2400: crate::Name = crate::Name::new_unchecked("MELP2400");
/// `MELP600`
pub const MELP600: crate::Name = crate::Name::new_unchecked("MELP600");
/// `merge-patch`
pub const MERGE_PATCH: crate::Name = crate::Name::new_unchecked("merge-patch");
/// `mesh`
pub const MESH: crate::Name = crate::Name::new_unchecked("mesh");
/// `message`
pub const MESSAGE: crate::Name = crate::Name::new_unchecked("message");
/// `metalink4`
pub const METALINK4: crate::Name = crate::Name::new_unchecked("metalink4");
/// `mets`
pub const METS: crate::Name = crate::Name::new_unchecked("mets");
/// `MF4`
pub const MF4: crate::Name = crate::Name::new_unchecked("MF4");
/// `mhas`
pub const MHAS: crate::Name = crate::Name::new_unchecked("mhas");
/// `mikey`
pub const MIKEY: crate::Name = crate::Name::new_unchecked("mikey");
/// `mipc`
pub const MIPC: crate::Name = crate::Name::new_unchecked("mipc");
/// `missing-blocks`
pub const MISSING_BLOCKS: crate::Name = crate::Name::new_unchecked("missing-blocks");
/// `mixed`
pub const MIXED: crate::Name = crate::Name::new_unchecked("mixed");
/// `mizar`
pub const MIZAR: crate::Name = crate::Name::new_unchecked("mizar");
/// `mj2`
pub const MJ2: crate::Name = crate::Name::new_unchecked("mj2");
/// `mmt-aei`
pub const MMT_AEI: crate::Name = crate::Name::new_unchecked("mmt-aei");
/// `mmt-usd`
pub const MMT_USD: crate::Name = crate::Name::new_unchecked("mmt-usd");
/// `mobile-xmf`
pub const MOBILE_XMF: crate::Name = crate::Name::new_unchecked("mobile-xmf");
/// `model`
pub const MODEL: crate::Name = crate::Name::new_unchecked("model");
/// `mods`
pub const MODS: crate::Name = crate::Name::new_unchecked("mods");
/// `moss-keys`
pub const MOSS_KEYS: crate::Name = crate::Name::new_unchecked("moss-keys");
/// `moss-signature`
pub const MOSS_SIGNATURE: crate::Name = crate::Name::new_unchecked("moss-signature");
/// `mosskey-data`
pub const MOSSKEY_DATA: crate::Name = crate::Name::new_unchecked("mosskey-data");
/// `mosskey-request`
pub const MOSSKEY_REQUEST: crate::Name = crate::Name::new_unchecked("mosskey-request");
/// `MP1S`
pub const MP1S: crate::Name = crate::Name::new_unchecked("MP1S");
/// `mp21`
pub const MP21: crate::Name = crate::Name::new_unchecked("mp21");
/// `MP2P`
pub const MP2P: crate::Name = crate::Name::new_unchecked("MP2P");
/// `mp2t`
pub const MP2T: crate::Name = crate::Name::new_unchecked("mp2t");
/// `mp4`
pub const MP4: crate::Name = crate::Name::new_unchecked("mp4");
/// `MP4A-LATM`
pub const MP4A_LATM: crate::Name = crate::Name::new_unchecked("MP4A-LATM");
/// `MP4V-ES`
pub const MP4V_ES: crate::Name = crate::Name::new_unchecked("MP4V-ES");
/// `MPA`
pub const MPA: crate::Name = crate::Name::new_unchecked("MPA");
/// `mpa-robust`
pub const MPA_ROBUST: crate::Name = crate::Name::new_unchecked("mpa-robust");
/// `mpeg`
pub const MPEG: crate::Name = crate::Name::new_unchecked("mpeg");
/// `mpeg4-generic`
pub const MPEG4_GENERIC: crate::Name = crate::Name::new_unchecked("mpeg4-generic");
/// `mpeg4-iod`
pub const MPEG4_IOD: crate::Name = crate::Name::new_unchecked("mpeg4-iod");
/// `mpeg4-iod-xmt`
pub const MPEG4_IOD_XMT: crate::Name = crate::Name::new_unchecked("mpeg4-iod-xmt");
/// `MPV`
pub const MPV: crate::Name = crate::Name::new_unchecked("MPV");
/// `mrb-consumer`
pub const MRB_CONSUMER: crate::Name = crate::Name::new_unchecked("mrb-consumer");
/// `mrb-publish`
pub const MRB_PUBLISH: crate::Name = crate::Name::new_unchecked("mrb-publish");
/// `msc-ivr`
pub const MSC_IVR: crate::Name = crate::Name::new_unchecked("msc-ivr");
/// `msc-mixer`
pub const MSC_MIXER: crate::Name = crate::Name::new_unchecked("msc-mixer");
/// `msgpack`
pub const MSGPACK: crate::Name = crate::Name::new_unchecked("msgpack");
/// `msword`
pub const MSWORD: crate::Name = crate::Name::new_unchecked("msword");
/// `mtl`
pub const MTL: crate::Name = crate::Name::new_unchecked("mtl");
/// `mud`
pub const MUD: crate::Name = crate::Name::new_unchecked("mud");
/// `multilingual`
pub const MULTILINGUAL: crate::Name = crate::Name::new_unchecked("multilingual");
/// `multipart`
pub const MULTIPART: crate::Name = crate::Name::new_unchecked("multipart");
/// `multipart-core`
pub const MULTIPART_CORE: crate::Name = crate::Name::new_unchecked("multipart-core");
/// `mxf`
pub const MXF: crate::Name = crate::Name::new_unchecked("mxf");
/// `n-quads`
pub const N_QUADS: crate::Name = crate::Name::new_unchecked("n-quads");
/// `n-triples`
pub const N_TRIPLES: crate::Name = crate::Name::new_unchecked("n-triples");
/// `n3`
pub const N3: crate::Name = crate::Name::new_unchecked("n3");
/// `naplps`
pub const NAPLPS: crate::Name = crate::Name::new_unchecked("naplps");
/// `nasdata`
pub const NASDATA: crate::Name = crate::Name::new_unchecked("nasdata");
/// `news-checkgroups`
pub const NEWS_CHECKGROUPS: crate::Name = crate::Name::new_unchecked("news-checkgroups");
/// `news-groupinfo`
pub const NEWS_GROUPINFO: crate::Name = crate::Name::new_unchecked("news-groupinfo");
/// `news-transmission`
pub const NEWS_TRANSMISSION: crate::Name = crate::Name::new_unchecked("news-transmission");
/// `nlsml`
pub const NLSML: crate::Name = crate::Name::new_unchecked("nlsml");
/// `node`
pub const NODE: crate::Name = crate::Name::new_unchecked("node");
/// `nss`
pub const NSS: crate::Name = crate::Name::new_unchecked("nss");
/// `nv`
pub const NV: crate::Name = crate::Name::new_unchecked("nv");
/// `oauth-authz-req`
pub const OAUTH_AUTHZ_REQ: crate::Name = crate::Name::new_unchecked("oauth-authz-req");
/// `obj`
pub const OBJ: crate::Name = crate::Name::new_unchecked("obj");
/// `ocsp-request`
pub const OCSP_REQUEST: crate::Name = crate::Name::new_unchecked("ocsp-request");
/// `ocsp-response`
pub const OCSP_RESPONSE: crate::Name = crate::Name::new_unchecked("ocsp-response");
/// `octet-stream`
pub const OCTET_STREAM: crate::Name = crate::Name::new_unchecked("octet-stream");
/// `ODA`
pub const ODA: crate::Name = crate::Name::new_unchecked("ODA");
/// `odm`
pub const ODM: crate::Name = crate::Name::new_unchecked("odm");
/// `ODX`
pub const ODX: crate::Name = crate::Name::new_unchecked("ODX");
/// `oebps-package`
pub const OEBPS_PACKAGE: crate::Name = crate::Name::new_unchecked("oebps-package");
/// `ogg`
pub const OGG: crate::Name = crate::Name::new_unchecked("ogg");
/// `opc-nodeset`
pub const OPC_NODESET: crate::Name = crate::Name::new_unchecked("opc-nodeset");
/// `opus`
pub const OPUS: crate::Name = crate::Name::new_unchecked("opus");
/// `oscore`
pub const OSCORE: crate::Name = crate::Name::new_unchecked("oscore");
/// `otf`
pub const OTF: crate::Name = crate::Name::new_unchecked("otf");
/// `oxps`
pub const OXPS: crate::Name = crate::Name::new_unchecked("oxps");
/// `p21`
pub const P21: crate::Name = crate::Name::new_unchecked("p21");
/// `p2p-overlay`
pub const P2P_OVERLAY: crate::Name = crate::Name::new_unchecked("p2p-overlay");
/// `parallel`
pub const PARALLEL: crate::Name = crate::Name::new_unchecked("parallel");
/// `parameters`
pub const PARAMETERS: crate::Name = crate::Name::new_unchecked("parameters");
/// `parityfec`
pub const PARITYFEC: crate::Name = crate::Name::new_unchecked("parityfec");
/// `passport`
pub const PASSPORT: crate::Name = crate::Name::new_unchecked("passport");
/// `patch-ops-error`
pub const PATCH_OPS_ERROR: crate::Name = crate::Name::new_unchecked("patch-ops-error");
/// `PCMA`
pub const PCMA: crate::Name = crate::Name::new_unchecked("PCMA");
/// `PCMA-WB`
pub const PCMA_WB: crate::Name = crate::Name::new_unchecked("PCMA-WB");
/// `PCMU`
pub const PCMU: crate::Name = crate::Name::new_unchecked("PCMU");
/// `PCMU-WB`
pub const PCMU_WB: crate::Name = crate::Name::new_unchecked("PCMU-WB");
/// `pdf`
pub const PDF: crate::Name = crate::Name::new_unchecked("pdf");
/// `PDX`
pub const PDX: crate::Name = crate::Name::new_unchecked("PDX");
/// `pem-certificate-chain`
pub const PEM_CERTIFICATE_CHAIN: crate::Name = crate::Name::new_unchecked("pem-certificate-chain");
/// `pgp-encrypted`
pub const PGP_ENCRYPTED: crate::Name = crate::Name::new_unchecked("pgp-encrypted");
/// `pgp-keys`
pub const PGP_KEYS: crate::Name = crate::Name::new_unchecked("pgp-keys");
/// `pgp-signature`
pub const PGP_SIGNATURE: crate::Name = crate::Name::new_unchecked("pgp-signature");
/// `pidf`
pub const PIDF: crate::Name = crate::Name::new_unchecked("pidf");
/// `pidf-diff`
pub const PIDF_DIFF: crate::Name = crate::Name::new_unchecked("pidf-diff");
/// `pkcs10`
pub const PKCS10: crate::Name = crate::Name::new_unchecked("pkcs10");
/// `pkcs12`
pub const PKCS12: crate::Name = crate::Name::new_unchecked("pkcs12");
/// `pkcs7-mime`
pub const PKCS7_MIME: crate::Name = crate::Name::new_unchecked("pkcs7-mime");
/// `pkcs7-signature`
pub const PKCS7_SIGNATURE: crate::Name = crate::Name::new_unchecked("pkcs7-signature");
/// `pkcs8`
pub const PKCS8: crate::Name = crate::Name::new_unchecked("pkcs8");
/// `pkcs8-encrypted`
pub const PKCS8_ENCRYPTED: crate::Name = crate::Name::new_unchecked("pkcs8-encrypted");
/// `pkix-attr-cert`
pub const PKIX_ATTR_CERT: crate::Name = crate::Name::new_unchecked("pkix-attr-cert");
/// `pkix-cert`
pub const PKIX_CERT: crate::Name = crate::Name::new_unchecked("pkix-cert");
/// `pkix-crl`
pub const PKIX_CRL: crate::Name = crate::Name::new_unchecked("pkix-crl");
/// `pkix-pkipath`
pub const PKIX_PKIPATH: crate::Name = crate::Name::new_unchecked("pkix-pkipath");
/// `pkixcmp`
pub const PKIXCMP: crate::Name = crate::Name::new_unchecked("pkixcmp");
/// `plain`
pub const PLAIN: crate::Name = crate::Name::new_unchecked("plain");
/// `pls`
pub const PLS: crate::Name = crate::Name::new_unchecked("pls");
/// `png`
pub const PNG: crate::Name = crate::Name::new_unchecked("png");
/// `poc-settings`
pub const POC_SETTINGS: crate::Name = crate::Name::new_unchecked("poc-settings");
/// `pointer`
pub const POINTER: crate::Name = crate::Name::new_unchecked("pointer");
/// `postscript`
pub const POSTSCRIPT: crate::Name = crate::Name::new_unchecked("postscript");
/// `ppsp-tracker`
pub const PPSP_TRACKER: crate::Name = crate::Name::new_unchecked("ppsp-tracker");
/// `problem`
pub const PROBLEM: crate::Name = crate::Name::new_unchecked("problem");
/// `provenance`
pub const PROVENANCE: crate::Name = crate::Name::new_unchecked("provenance");
/// `provenance-notation`
pub const PROVENANCE_NOTATION: crate::Name = crate::Name::new_unchecked("provenance-notation");
/// `prs.alvestrand.titrax-sheet`
pub const PRS_ALVESTRAND_TITRAX_SHEET: crate::Name = crate::Name::new_unchecked("prs.alvestrand.titrax-sheet");
/// `prs.btif`
pub const PRS_BTIF: crate::Name = crate::Name::new_unchecked("prs.btif");
/// `prs.cww`
pub const PRS_CWW: crate::Name = crate::Name::new_unchecked("prs.cww");
/// `prs.cyn`
pub const PRS_CYN: crate::Name = crate::Name::new_unchecked("prs.cyn");
/// `prs.fallenstein.rst`
pub const PRS_FALLENSTEIN_RST: crate::Name = crate::Name::new_unchecked("prs.fallenstein.rst");
/// `prs.hpub`
pub const PRS_HPUB: crate::Name = crate::Name::new_unchecked("prs.hpub");
/// `prs.lines.tag`
pub const PRS_LINES_TAG: crate::Name = crate::Name::new_unchecked("prs.lines.tag");
/// `prs.nprend`
pub const PRS_NPREND: crate::Name = crate::Name::new_unchecked("prs.nprend");
/// `prs.plucker`
pub const PRS_PLUCKER: crate::Name = crate::Name::new_unchecked("prs.plucker");
/// `prs.prop.logic`
pub const PRS_PROP_LOGIC: crate::Name = crate::Name::new_unchecked("prs.prop.logic");
/// `prs.pti`
pub const PRS_PTI: crate::Name = crate::Name::new_unchecked("prs.pti");
/// `prs.rdf-xml-crypt`
pub const PRS_RDF_XML_CRYPT: crate::Name = crate::Name::new_unchecked("prs.rdf-xml-crypt");
/// `prs.sid`
pub const PRS_SID: crate::Name = crate::Name::new_unchecked("prs.sid");
/// `prs.xsf`
pub const PRS_XSF: crate::Name = crate::Name::new_unchecked("prs.xsf");
/// `pskc`
pub const PSKC: crate::Name = crate::Name::new_unchecked("pskc");
/// `pvd`
pub const PVD: crate::Name = crate::Name::new_unchecked("pvd");
/// `pwg-raster`
pub const PWG_RASTER: crate::Name = crate::Name::new_unchecked("pwg-raster");
/// `q`
pub const Q: crate::Name = crate::Name::new_unchecked("q");
/// `QCELP`
pub const QCELP: crate::Name = crate::Name::new_unchecked("QCELP");
/// `QSIG`
pub const QSIG: crate::Name = crate::Name::new_unchecked("QSIG");
/// `quicktime`
pub const QUICKTIME: crate::Name = crate::Name::new_unchecked("quicktime");
/// `raptorfec`
pub const RAPTORFEC: crate::Name = crate::Name::new_unchecked("raptorfec");
/// `raw`
pub const RAW: crate::Name = crate::Name::new_unchecked("raw");
/// `rdap`
pub const RDAP: crate::Name = crate::Name::new_unchecked("rdap");
/// `rdf`
pub const RDF: crate::Name = crate::Name::new_unchecked("rdf");
/// `RED`
pub const RED: crate::Name = crate::Name::new_unchecked("RED");
/// `reginfo`
pub const REGINFO: crate::Name = crate::Name::new_unchecked("reginfo");
/// `related`
pub const RELATED: crate::Name = crate::Name::new_unchecked("related");
/// `relax-ng-compact-syntax`
pub const RELAX_NG_COMPACT_SYNTAX: crate::Name = crate::Name::new_unchecked("relax-ng-compact-syntax");
/// `remote-printing`
pub const REMOTE_PRINTING: crate::Name = crate::Name::new_unchecked("remote-printing");
/// `report`
pub const REPORT: crate::Name = crate::Name::new_unchecked("report");
/// `reputon`
pub const REPUTON: crate::Name = crate::Name::new_unchecked("reputon");
/// `resource-lists`
pub const RESOURCE_LISTS: crate::Name = crate::Name::new_unchecked("resource-lists");
/// `resource-lists-diff`
pub const RESOURCE_LISTS_DIFF: crate::Name = crate::Name::new_unchecked("resource-lists-diff");
/// `rfc`
pub const RFC: crate::Name = crate::Name::new_unchecked("rfc");
/// `rfc822-headers`
pub const RFC822_HEADERS: crate::Name = crate::Name::new_unchecked("rfc822-headers");
/// `richtext`
pub const RICHTEXT: crate::Name = crate::Name::new_unchecked("richtext");
/// `riscos`
pub const RISCOS: crate::Name = crate::Name::new_unchecked("riscos");
/// `rlmi`
pub const RLMI: crate::Name = crate::Name::new_unchecked("rlmi");
/// `rls-services`
pub const RLS_SERVICES: crate::Name = crate::Name::new_unchecked("rls-services");
/// `route-apd`
pub const ROUTE_APD: crate::Name = crate::Name::new_unchecked("route-apd");
/// `route-s-tsid`
pub const ROUTE_S_TSID: crate::Name = crate::Name::new_unchecked("route-s-tsid");
/// `route-usd`
pub const ROUTE_USD: crate::Name = crate::Name::new_unchecked("route-usd");
/// `rpki-ghostbusters`
pub const RPKI_GHOSTBUSTERS: crate::Name = crate::Name::new_unchecked("rpki-ghostbusters");
/// `rpki-manifest`
pub const RPKI_MANIFEST: crate::Name = crate::Name::new_unchecked("rpki-manifest");
/// `rpki-publication`
pub const RPKI_PUBLICATION: crate::Name = crate::Name::new_unchecked("rpki-publication");
/// `rpki-roa`
pub const RPKI_ROA: crate::Name = crate::Name::new_unchecked("rpki-roa");
/// `rpki-updown`
pub const RPKI_UPDOWN: crate::Name = crate::Name::new_unchecked("rpki-updown");
/// `rtf`
pub const RTF: crate::Name = crate::Name::new_unchecked("rtf");
/// `rtp-enc-aescm128`
pub const RTP_ENC_AESCM128: crate::Name = crate::Name::new_unchecked("rtp-enc-aescm128");
/// `rtp-midi`
pub const RTP_MIDI: crate::Name = crate::Name::new_unchecked("rtp-midi");
/// `rtploopback`
pub const RTPLOOPBACK: crate::Name = crate::Name::new_unchecked("rtploopback");
/// `rtx`
pub const RTX: crate::Name = crate::Name::new_unchecked("rtx");
/// `samlassertion`
pub const SAMLASSERTION: crate::Name = crate::Name::new_unchecked("samlassertion");
/// `samlmetadata`
pub const SAMLMETADATA: crate::Name = crate::Name::new_unchecked("samlmetadata");
/// `sarif`
pub const SARIF: crate::Name = crate::Name::new_unchecked("sarif");
/// `sarif-external-properties`
pub const SARIF_EXTERNAL_PROPERTIES: crate::Name = crate::Name::new_unchecked("sarif-external-properties");
/// `sbe`
pub const SBE: crate::Name = crate::Name::new_unchecked("sbe");
/// `sbml`
pub const SBML: crate::Name = crate::Name::new_unchecked("sbml");
/// `scaip`
pub const SCAIP: crate::Name = crate::Name::new_unchecked("scaip");
/// `scim`
pub const SCIM: crate::Name = crate::Name::new_unchecked("scim");
/// `scip`
pub const SCIP: crate::Name = crate::Name::new_unchecked("scip");
/// `scvp-cv-request`
pub const SCVP_CV_REQUEST: crate::Name = crate::Name::new_unchecked("scvp-cv-request");
/// `scvp-cv-response`
pub const SCVP_CV_RESPONSE: crate::Name = crate::Name::new_unchecked("scvp-cv-response");
/// `scvp-vp-request`
pub const SCVP_VP_REQUEST: crate::Name = crate::Name::new_unchecked("scvp-vp-request");
/// `scvp-vp-response`
pub const SCVP_VP_RESPONSE: crate::Name = crate::Name::new_unchecked("scvp-vp-response");
/// `sdp`
pub const SDP: crate::Name = crate::Name::new_unchecked("sdp");
/// `secevent`
pub const SECEVENT: crate::Name = crate::Name::new_unchecked("secevent");
/// `senml`
pub const SENML: crate::Name = crate::Name::new_unchecked("senml");
/// `senml-etch`
pub const SENML_ETCH: crate::Name = crate::Name::new_unchecked("senml-etch");
/// `senml-exi`
pub const SENML_EXI: crate::Name = crate::Name::new_unchecked("senml-exi");
/// `sensml`
pub const SENSML: crate::Name = crate::Name::new_unchecked("sensml");
/// `sensml-exi`
pub const SENSML_EXI: crate::Name = crate::Name::new_unchecked("sensml-exi");
/// `sep`
pub const SEP: crate::Name = crate::Name::new_unchecked("sep");
/// `sep-exi`
pub const SEP_EXI: crate::Name = crate::Name::new_unchecked("sep-exi");
/// `session-info`
pub const SESSION_INFO: crate::Name = crate::Name::new_unchecked("session-info");
/// `set-payment`
pub const SET_PAYMENT: crate::Name = crate::Name::new_unchecked("set-payment");
/// `set-payment-initiation`
pub const SET_PAYMENT_INITIATION: crate::Name = crate::Name::new_unchecked("set-payment-initiation");
/// `set-registration`
pub const SET_REGISTRATION: crate::Name = crate::Name::new_unchecked("set-registration");
/// `set-registration-initiation`
pub const SET_REGISTRATION_INITIATION: crate::Name = crate::Name::new_unchecked("set-registration-initiation");
/// `sfnt`
pub const SFNT: crate::Name = crate::Name::new_unchecked("sfnt");
/// `SGML`
pub const SGML: crate::Name = crate::Name::new_unchecked("SGML");
/// `sgml-open-catalog`
pub const SGML_OPEN_CATALOG: crate::Name = crate::Name::new_unchecked("sgml-open-catalog");
/// `shaclc`
pub const SHACLC: crate::Name = crate::Name::new_unchecked("shaclc");
/// `shex`
pub const SHEX: crate::Name = crate::Name::new_unchecked("shex");
/// `shf`
pub const SHF: crate::Name = crate::Name::new_unchecked("shf");
/// `sieve`
pub const SIEVE: crate::Name = crate::Name::new_unchecked("sieve");
/// `signed`
pub const SIGNED: crate::Name = crate::Name::new_unchecked("signed");
/// `simple-filter`
pub const SIMPLE_FILTER: crate::Name = crate::Name::new_unchecked("simple-filter");
/// `simple-message-summary`
pub const SIMPLE_MESSAGE_SUMMARY: crate::Name = crate::Name::new_unchecked("simple-message-summary");
/// `simpleSymbolContainer`
pub const SIMPLE_SYMBOL_CONTAINER: crate::Name = crate::Name::new_unchecked("simpleSymbolContainer");
/// `sipc`
pub const SIPC: crate::Name = crate::Name::new_unchecked("sipc");
/// `slate`
pub const SLATE: crate::Name = crate::Name::new_unchecked("slate");
/// `smil`
pub const SMIL: crate::Name = crate::Name::new_unchecked("smil");
/// `smpte291`
pub const SMPTE291: crate::Name = crate::Name::new_unchecked("smpte291");
/// `SMPTE292M`
pub const SMPTE292M: crate::Name = crate::Name::new_unchecked("SMPTE292M");
/// `smpte336m`
pub const SMPTE336M: crate::Name = crate::Name::new_unchecked("smpte336m");
/// `SMV`
pub const SMV: crate::Name = crate::Name::new_unchecked("SMV");
/// `SMV-QCP`
pub const SMV_QCP: crate::Name = crate::Name::new_unchecked("SMV-QCP");
/// `SMV0`
pub const SMV0: crate::Name = crate::Name::new_unchecked("SMV0");
/// `soap`
pub const SOAP: crate::Name = crate::Name::new_unchecked("soap");
/// `sofa`
pub const SOFA: crate::Name = crate::Name::new_unchecked("sofa");
/// `sp-midi`
pub const SP_MIDI: crate::Name = crate::Name::new_unchecked("sp-midi");
/// `sparql-query`
pub const SPARQL_QUERY: crate::Name = crate::Name::new_unchecked("sparql-query");
/// `sparql-update`
pub const SPARQL_UPDATE: crate::Name = crate::Name::new_unchecked("sparql-update");
/// `sparql-results`
pub const SPARQL_RESULTS: crate::Name = crate::Name::new_unchecked("sparql-results");
/// `spdx`
pub const SPDX: crate::Name = crate::Name::new_unchecked("spdx");
/// `speex`
pub const SPEEX: crate::Name = crate::Name::new_unchecked("speex");
/// `spirits-event`
pub const SPIRITS_EVENT: crate::Name = crate::Name::new_unchecked("spirits-event");
/// `sql`
pub const SQL: crate::Name = crate::Name::new_unchecked("sql");
/// `srgs`
pub const SRGS: crate::Name = crate::Name::new_unchecked("srgs");
/// `sru`
pub const SRU: crate::Name = crate::Name::new_unchecked("sru");
/// `ssml`
pub const SSML: crate::Name = crate::Name::new_unchecked("ssml");
/// `step`
pub const STEP: crate::Name = crate::Name::new_unchecked("step");
/// `step-xml`
pub const STEP_XML: crate::Name = crate::Name::new_unchecked("step-xml");
/// `stix`
pub const STIX: crate::Name = crate::Name::new_unchecked("stix");
/// `stl`
pub const STL: crate::Name = crate::Name::new_unchecked("stl");
/// `strings`
pub const STRINGS: crate::Name = crate::Name::new_unchecked("strings");
/// `svg`
pub const SVG: crate::Name = crate::Name::new_unchecked("svg");
/// `swid`
pub const SWID: crate::Name = crate::Name::new_unchecked("swid");
/// `t140`
pub const T140: crate::Name = crate::Name::new_unchecked("t140");
/// `t140c`
pub const T140C: crate::Name = crate::Name::new_unchecked("t140c");
/// `t38`
pub const T38: crate::Name = crate::Name::new_unchecked("t38");
/// `tab-separated-values`
pub const TAB_SEPARATED_VALUES: crate::Name = crate::Name::new_unchecked("tab-separated-values");
/// `tamp-apex-update`
pub const TAMP_APEX_UPDATE: crate::Name = crate::Name::new_unchecked("tamp-apex-update");
/// `tamp-apex-update-confirm`
pub const TAMP_APEX_UPDATE_CONFIRM: crate::Name = crate::Name::new_unchecked("tamp-apex-update-confirm");
/// `tamp-community-update`
pub const TAMP_COMMUNITY_UPDATE: crate::Name = crate::Name::new_unchecked("tamp-community-update");
/// `tamp-community-update-confirm`
pub const TAMP_COMMUNITY_UPDATE_CONFIRM: crate::Name = crate::Name::new_unchecked("tamp-community-update-confirm");
/// `tamp-error`
pub const TAMP_ERROR: crate::Name = crate::Name::new_unchecked("tamp-error");
/// `tamp-sequence-adjust`
pub const TAMP_SEQUENCE_ADJUST: crate::Name = crate::Name::new_unchecked("tamp-sequence-adjust");
/// `tamp-sequence-adjust-confirm`
pub const TAMP_SEQUENCE_ADJUST_CONFIRM: crate::Name = crate::Name::new_unchecked("tamp-sequence-adjust-confirm");
/// `tamp-status-query`
pub const TAMP_STATUS_QUERY: crate::Name = crate::Name::new_unchecked("tamp-status-query");
/// `tamp-status-response`
pub const TAMP_STATUS_RESPONSE: crate::Name = crate::Name::new_unchecked("tamp-status-response");
/// `tamp-update`
pub const TAMP_UPDATE: crate::Name = crate::Name::new_unchecked("tamp-update");
/// `tamp-update-confirm`
pub const TAMP_UPDATE_CONFIRM: crate::Name = crate::Name::new_unchecked("tamp-update-confirm");
/// `taxii`
pub const TAXII: crate::Name = crate::Name::new_unchecked("taxii");
/// `td`
pub const TD: crate::Name = crate::Name::new_unchecked("td");
/// `tei`
pub const TEI: crate::Name = crate::Name::new_unchecked("tei");
/// `telephone-event`
pub const TELEPHONE_EVENT: crate::Name = crate::Name::new_unchecked("telephone-event");
/// `TETRA_ACELP`
pub const TETRA_ACELP: crate::Name = crate::Name::new_unchecked("TETRA_ACELP");
/// `TETRA_ACELP_BB`
pub const TETRA_ACELP_BB: crate::Name = crate::Name::new_unchecked("TETRA_ACELP_BB");
/// `TETRA_ISI`
pub const TETRA_ISI: crate::Name = crate::Name::new_unchecked("TETRA_ISI");
/// `text`
pub const TEXT: crate::Name = crate::Name::new_unchecked("text");
/// `thraud`
pub const THRAUD: crate::Name = crate::Name::new_unchecked("thraud");
/// `tiff`
pub const TIFF: crate::Name = crate::Name::new_unchecked("tiff");
/// `tiff-fx`
pub const TIFF_FX: crate::Name = crate::Name::new_unchecked("tiff-fx");
/// `timestamp-query`
pub const TIMESTAMP_QUERY: crate::Name = crate::Name::new_unchecked("timestamp-query");
/// `timestamp-reply`
pub const TIMESTAMP_REPLY: crate::Name = crate::Name::new_unchecked("timestamp-reply");
/// `timestamped-data`
pub const TIMESTAMPED_DATA: crate::Name = crate::Name::new_unchecked("timestamped-data");
/// `tlsrpt`
pub const TLSRPT: crate::Name = crate::Name::new_unchecked("tlsrpt");
/// `tnauthlist`
pub const TNAUTHLIST: crate::Name = crate::Name::new_unchecked("tnauthlist");
/// `token-introspection`
pub const TOKEN_INTROSPECTION: crate::Name = crate::Name::new_unchecked("token-introspection");
/// `tone`
pub const TONE: crate::Name = crate::Name::new_unchecked("tone");
/// `trickle-ice-sdpfrag`
pub const TRICKLE_ICE_SDPFRAG: crate::Name = crate::Name::new_unchecked("trickle-ice-sdpfrag");
/// `trig`
pub const TRIG: crate::Name = crate::Name::new_unchecked("trig");
/// `troff`
pub const TROFF: crate::Name = crate::Name::new_unchecked("troff");
/// `TSVCIS`
pub const TSVCIS: crate::Name = crate::Name::new_unchecked("TSVCIS");
/// `ttf`
pub const TTF: crate::Name = crate::Name::new_unchecked("ttf");
/// `ttml`
pub const TTML: crate::Name = crate::Name::new_unchecked("ttml");
/// `turtle`
pub const TURTLE: crate::Name = crate::Name::new_unchecked("turtle");
/// `tve-trigger`
pub const TVE_TRIGGER: crate::Name = crate::Name::new_unchecked("tve-trigger");
/// `tzif`
pub const TZIF: crate::Name = crate::Name::new_unchecked("tzif");
/// `tzif-leap`
pub const TZIF_LEAP: crate::Name = crate::Name::new_unchecked("tzif-leap");
/// `UEMCLIP`
pub const UEMCLIP: crate::Name = crate::Name::new_unchecked("UEMCLIP");
/// `ulpfec`
pub const ULPFEC: crate::Name = crate::Name::new_unchecked("ulpfec");
/// `urc-grpsheet`
pub const URC_GRPSHEET: crate::Name = crate::Name::new_unchecked("urc-grpsheet");
/// `urc-ressheet`
pub const URC_RESSHEET: crate::Name = crate::Name::new_unchecked("urc-ressheet");
/// `urc-targetdesc`
pub const URC_TARGETDESC: crate::Name = crate::Name::new_unchecked("urc-targetdesc");
/// `urc-uisocketdesc`
pub const URC_UISOCKETDESC: crate::Name = crate::Name::new_unchecked("urc-uisocketdesc");
/// `uri-list`
pub const URI_LIST: crate::Name = crate::Name::new_unchecked("uri-list");
/// `usac`
pub const USAC: crate::Name = crate::Name::new_unchecked("usac");
/// `vc1`
pub const VC1: crate::Name = crate::Name::new_unchecked("vc1");
/// `vc2`
pub const VC2: crate::Name = crate::Name::new_unchecked("vc2");
/// `vcard`
pub const VCARD: crate::Name = crate::Name::new_unchecked("vcard");
/// `VDVI`
pub const VDVI: crate::Name = crate::Name::new_unchecked("VDVI");
/// `vemmi`
pub const VEMMI: crate::Name = crate::Name::new_unchecked("vemmi");
/// `video`
pub const VIDEO: crate::Name = crate::Name::new_unchecked("video");
/// `VMR-WB`
pub const VMR_WB: crate::Name = crate::Name::new_unchecked("VMR-WB");
/// `voice-message`
pub const VOICE_MESSAGE: crate::Name = crate::Name::new_unchecked("voice-message");
/// `voicexml`
pub const VOICEXML: crate::Name = crate::Name::new_unchecked("voicexml");
/// `vorbis`
pub const VORBIS: crate::Name = crate::Name::new_unchecked("vorbis");
/// `vorbis-config`
pub const VORBIS_CONFIG: crate::Name = crate::Name::new_unchecked("vorbis-config");
/// `voucher-cms`
pub const VOUCHER_CMS: crate::Name = crate::Name::new_unchecked("voucher-cms");
/// `VP8`
pub const VP8: crate::Name = crate::Name::new_unchecked("VP8");
/// `VP9`
pub const VP9: crate::Name = crate::Name::new_unchecked("VP9");
/// `vq-rtcpxr`
pub const VQ_RTCPXR: crate::Name = crate::Name::new_unchecked("vq-rtcpxr");
/// `vrml`
pub const VRML: crate::Name = crate::Name::new_unchecked("vrml");
/// `vtt`
pub const VTT: crate::Name = crate::Name::new_unchecked("vtt");
/// `wasm`
pub const WASM: crate::Name = crate::Name::new_unchecked("wasm");
/// `watcherinfo`
pub const WATCHERINFO: crate::Name = crate::Name::new_unchecked("watcherinfo");
/// `wav`
pub const WAV: crate::Name = crate::Name::new_unchecked("wav");
/// `wbxml`
pub const WBXML: crate::Name = crate::Name::new_unchecked("wbxml");
/// `webm`
pub const WEBM: crate::Name = crate::Name::new_unchecked("webm");
/// `webp`
pub const WEBP: crate::Name = crate::Name::new_unchecked("webp");
/// `webpush-options`
pub const WEBPUSH_OPTIONS: crate::Name = crate::Name::new_unchecked("webpush-options");
/// `whoispp-query`
pub const WHOISPP_QUERY: crate::Name = crate::Name::new_unchecked("whoispp-query");
/// `whoispp-response`
pub const WHOISPP_RESPONSE: crate::Name = crate::Name::new_unchecked("whoispp-response");
/// `widget`
pub const WIDGET: crate::Name = crate::Name::new_unchecked("widget");
/// `wita`
pub const WITA: crate::Name = crate::Name::new_unchecked("wita");
/// `wmf`
pub const WMF: crate::Name = crate::Name::new_unchecked("wmf");
/// `woff`
pub const WOFF: crate::Name = crate::Name::new_unchecked("woff");
/// `woff2`
pub const WOFF2: crate::Name = crate::Name::new_unchecked("woff2");
/// `wordperfect5.1`
pub const WORDPERFECT5_1: crate::Name = crate::Name::new_unchecked("wordperfect5.1");
/// `wsdl`
pub const WSDL: crate::Name = crate::Name::new_unchecked("wsdl");
/// `wspolicy`
pub const WSPOLICY: crate::Name = crate::Name::new_unchecked("wspolicy");
/// `x3d`
pub const X3D: crate::Name = crate::Name::new_unchecked("x3d");
/// `x3d-vrml`
pub const X3D_VRML: crate::Name = crate::Name::new_unchecked("x3d-vrml");
/// `x400-bp`
pub const X400_BP: crate::Name = crate::Name::new_unchecked("x400-bp");
/// `xacml`
pub const XACML: crate::Name = crate::Name::new_unchecked("xacml");
/// `xcap-att`
pub const XCAP_ATT: crate::Name = crate::Name::new_unchecked("xcap-att");
/// `xcap-caps`
pub const XCAP_CAPS: crate::Name = crate::Name::new_unchecked("xcap-caps");
/// `xcap-diff`
pub const XCAP_DIFF: crate::Name = crate::Name::new_unchecked("xcap-diff");
/// `xcap-el`
pub const XCAP_EL: crate::Name = crate::Name::new_unchecked("xcap-el");
/// `xcap-error`
pub const XCAP_ERROR: crate::Name = crate::Name::new_unchecked("xcap-error");
/// `xcap-ns`
pub const XCAP_NS: crate::Name = crate::Name::new_unchecked("xcap-ns");
/// `xcon-conference-info`
pub const XCON_CONFERENCE_INFO: crate::Name = crate::Name::new_unchecked("xcon-conference-info");
/// `xcon-conference-info-diff`
pub const XCON_CONFERENCE_INFO_DIFF: crate::Name = crate::Name::new_unchecked("xcon-conference-info-diff");
/// `xenc`
pub const XENC: crate::Name = crate::Name::new_unchecked("xenc");
/// `xhtml`
pub const XHTML: crate::Name = crate::Name::new_unchecked("xhtml");
/// `xliff`
pub const XLIFF: crate::Name = crate::Name::new_unchecked("xliff");
/// `xml`
pub const XML: crate::Name = crate::Name::new_unchecked("xml");
/// `xml-dtd`
pub const XML_DTD: crate::Name = crate::Name::new_unchecked("xml-dtd");
/// `xml-external-parsed-entity`
pub const XML_EXTERNAL_PARSED_ENTITY: crate::Name = crate::Name::new_unchecked("xml-external-parsed-entity");
/// `xml-patch`
pub const XML_PATCH: crate::Name = crate::Name::new_unchecked("xml-patch");
/// `xmpp`
pub const XMPP: crate::Name = crate::Name::new_unchecked("xmpp");
/// `xop`
pub const XOP: crate::Name = crate::Name::new_unchecked("xop");
/// `xslt`
pub const XSLT: crate::Name = crate::Name::new_unchecked("xslt");
/// `xv`
pub const XV: crate::Name = crate::Name::new_unchecked("xv");
/// `yang`
pub const YANG: crate::Name = crate::Name::new_unchecked("yang");
/// `yang-data`
pub const YANG_DATA: crate::Name = crate::Name::new_unchecked("yang-data");
/// `yang-patch`
pub const YANG_PATCH: crate::Name = crate::Name::new_unchecked("yang-patch");
/// `yin`
pub const YIN: crate::Name = crate::Name::new_unchecked("yin");
/// `zip`
pub const ZIP: crate::Name = crate::Name::new_unchecked("zip");
/// `zlib`
pub const ZLIB: crate::Name = crate::Name::new_unchecked("zlib");
/// `zstd`
pub const ZSTD: crate::Name = crate::Name::new_unchecked("zstd");
