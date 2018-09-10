// This file was generated by gir (https://github.com/gtk-rs/gir @ f5fca82)
// from gir-files (https://github.com/gtk-rs/gir-files @ b8f5ef1)
// DO NOT EDIT

extern crate webkit2gtk_sys;
extern crate shell_words;
extern crate tempdir;
use std::env;
use std::error::Error;
use std::path::Path;
use std::mem::{align_of, size_of};
use std::process::Command;
use std::str;
use webkit2gtk_sys::*;

static PACKAGES: &[&str] = &["webkit2gtk-4.0"];

#[derive(Clone, Debug)]
struct Compiler {
    pub args: Vec<String>,
}

impl Compiler {
    pub fn new() -> Result<Compiler, Box<Error>> {
        let mut args = get_var("CC", "cc")?;
        args.push("-Wno-deprecated-declarations".to_owned());
        // For %z support in printf when using MinGW.
        args.push("-D__USE_MINGW_ANSI_STDIO".to_owned());
        args.extend(get_var("CFLAGS", "")?);
        args.extend(get_var("CPPFLAGS", "")?);
        args.extend(pkg_config_cflags(PACKAGES)?);
        Ok(Compiler { args })
    }

    pub fn define<'a, V: Into<Option<&'a str>>>(&mut self, var: &str, val: V) {
        let arg = match val.into() {
            None => format!("-D{}", var),
            Some(val) => format!("-D{}={}", var, val),
        };
        self.args.push(arg);
    }

    pub fn compile(&self, src: &Path, out: &Path) -> Result<(), Box<Error>> {
        let mut cmd = self.to_command();
        cmd.arg(src);
        cmd.arg("-o");
        cmd.arg(out);
        let status = cmd.spawn()?.wait()?;
        if !status.success() {
            return Err(format!("compilation command {:?} failed, {}",
                               &cmd, status).into());
        }
        Ok(())
    }

    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.args[0]);
        cmd.args(&self.args[1..]);
        cmd
    }
}

fn get_var(name: &str, default: &str) -> Result<Vec<String>, Box<Error>> {
    match env::var(name) {
        Ok(value) => Ok(shell_words::split(&value)?),
        Err(env::VarError::NotPresent) => Ok(shell_words::split(default)?),
        Err(err) => Err(format!("{} {}", name, err).into()),
    }
}

fn pkg_config_cflags(packages: &[&str]) -> Result<Vec<String>, Box<Error>> {
    if packages.is_empty() {
        return Ok(Vec::new());
    }
    let mut cmd = Command::new("pkg-config");
    cmd.arg("--cflags");
    cmd.args(packages);
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(format!("command {:?} returned {}",
                           &cmd, out.status).into());
    }
    let stdout = str::from_utf8(&out.stdout)?;
    Ok(shell_words::split(stdout.trim())?)
}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Layout {
    size: usize,
    alignment: usize,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Results {
    /// Number of successfully completed tests.
    passed: usize,
    /// Total number of failed tests (including those that failed to compile).
    failed: usize,
    /// Number of tests that failed to compile.
    failed_to_compile: usize,
}

impl Results {
    fn record_passed(&mut self) {
        self.passed += 1;
    }
    fn record_failed(&mut self) {
        self.failed += 1;
    }
    fn record_failed_to_compile(&mut self) {
        self.failed += 1;
        self.failed_to_compile += 1;
    }
    fn summary(&self) -> String {
        format!(
            "{} passed; {} failed (compilation errors: {})",
            self.passed,
            self.failed,
            self.failed_to_compile)
    }
    fn expect_total_success(&self) {
        if self.failed == 0 {
            println!("OK: {}", self.summary());
        } else {
            panic!("FAILED: {}", self.summary());
        };
    }
}

#[test]
fn cross_validate_constants_with_c() {
    let tmpdir = tempdir::TempDir::new("abi").expect("temporary directory");
    let cc = Compiler::new().expect("configured compiler");

    assert_eq!("1",
               get_c_value(tmpdir.path(), &cc, "1").expect("C constant"),
               "failed to obtain correct constant value for 1");

    let mut results : Results = Default::default();
    for (i, &(name, rust_value)) in RUST_CONSTANTS.iter().enumerate() {
        match get_c_value(tmpdir.path(), &cc, name) {
            Err(e) => {
                results.record_failed_to_compile();
                eprintln!("{}", e);
            },
            Ok(ref c_value) => {
                if rust_value == c_value {
                    results.record_passed();
                } else {
                    results.record_failed();
                    eprintln!("Constant value mismatch for {}\nRust: {:?}\nC:    {:?}",
                              name, rust_value, c_value);
                }
            }
        };
        if (i + 1) % 25 == 0 {
            println!("constants ... {}", results.summary());
        }
    }
    results.expect_total_success();
}

#[test]
fn cross_validate_layout_with_c() {
    let tmpdir = tempdir::TempDir::new("abi").expect("temporary directory");
    let cc = Compiler::new().expect("configured compiler");

    assert_eq!(Layout {size: 1, alignment: 1},
               get_c_layout(tmpdir.path(), &cc, "char").expect("C layout"),
               "failed to obtain correct layout for char type");

    let mut results : Results = Default::default();
    for (i, &(name, rust_layout)) in RUST_LAYOUTS.iter().enumerate() {
        match get_c_layout(tmpdir.path(), &cc, name) {
            Err(e) => {
                results.record_failed_to_compile();
                eprintln!("{}", e);
            },
            Ok(c_layout) => {
                if rust_layout == c_layout {
                    results.record_passed();
                } else {
                    results.record_failed();
                    eprintln!("Layout mismatch for {}\nRust: {:?}\nC:    {:?}",
                              name, rust_layout, &c_layout);
                }
            }
        };
        if (i + 1) % 25 == 0 {
            println!("layout    ... {}", results.summary());
        }
    }
    results.expect_total_success();
}

fn get_c_layout(dir: &Path, cc: &Compiler, name: &str) -> Result<Layout, Box<Error>> {
    let exe = dir.join("layout");
    let mut cc = cc.clone();
    cc.define("ABI_TYPE_NAME", name);
    cc.compile(Path::new("tests/layout.c"), &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}",
                           &abi_cmd, &output).into());
    }

    let stdout = str::from_utf8(&output.stdout)?;
    let mut words = stdout.trim().split_whitespace();
    let size = words.next().unwrap().parse().unwrap();
    let alignment = words.next().unwrap().parse().unwrap();
    Ok(Layout {size, alignment})
}

fn get_c_value(dir: &Path, cc: &Compiler, name: &str) -> Result<String, Box<Error>> {
    let exe = dir.join("constant");
    let mut cc = cc.clone();
    cc.define("ABI_CONSTANT_NAME", name);
    cc.compile(Path::new("tests/constant.c"), &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}",
                           &abi_cmd, &output).into());
    }

    Ok(str::from_utf8(&output.stdout)?.trim().to_owned())
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[
    ("WebKitAuthenticationRequest", Layout {size: size_of::<WebKitAuthenticationRequest>(), alignment: align_of::<WebKitAuthenticationRequest>()}),
    ("WebKitAuthenticationRequestClass", Layout {size: size_of::<WebKitAuthenticationRequestClass>(), alignment: align_of::<WebKitAuthenticationRequestClass>()}),
    ("WebKitAuthenticationScheme", Layout {size: size_of::<WebKitAuthenticationScheme>(), alignment: align_of::<WebKitAuthenticationScheme>()}),
    ("WebKitAutomationSession", Layout {size: size_of::<WebKitAutomationSession>(), alignment: align_of::<WebKitAutomationSession>()}),
    ("WebKitAutomationSessionClass", Layout {size: size_of::<WebKitAutomationSessionClass>(), alignment: align_of::<WebKitAutomationSessionClass>()}),
    ("WebKitBackForwardList", Layout {size: size_of::<WebKitBackForwardList>(), alignment: align_of::<WebKitBackForwardList>()}),
    ("WebKitBackForwardListClass", Layout {size: size_of::<WebKitBackForwardListClass>(), alignment: align_of::<WebKitBackForwardListClass>()}),
    ("WebKitBackForwardListItem", Layout {size: size_of::<WebKitBackForwardListItem>(), alignment: align_of::<WebKitBackForwardListItem>()}),
    ("WebKitBackForwardListItemClass", Layout {size: size_of::<WebKitBackForwardListItemClass>(), alignment: align_of::<WebKitBackForwardListItemClass>()}),
    ("WebKitCacheModel", Layout {size: size_of::<WebKitCacheModel>(), alignment: align_of::<WebKitCacheModel>()}),
    ("WebKitColorChooserRequest", Layout {size: size_of::<WebKitColorChooserRequest>(), alignment: align_of::<WebKitColorChooserRequest>()}),
    ("WebKitColorChooserRequestClass", Layout {size: size_of::<WebKitColorChooserRequestClass>(), alignment: align_of::<WebKitColorChooserRequestClass>()}),
    ("WebKitContextMenu", Layout {size: size_of::<WebKitContextMenu>(), alignment: align_of::<WebKitContextMenu>()}),
    ("WebKitContextMenuAction", Layout {size: size_of::<WebKitContextMenuAction>(), alignment: align_of::<WebKitContextMenuAction>()}),
    ("WebKitContextMenuClass", Layout {size: size_of::<WebKitContextMenuClass>(), alignment: align_of::<WebKitContextMenuClass>()}),
    ("WebKitContextMenuItem", Layout {size: size_of::<WebKitContextMenuItem>(), alignment: align_of::<WebKitContextMenuItem>()}),
    ("WebKitContextMenuItemClass", Layout {size: size_of::<WebKitContextMenuItemClass>(), alignment: align_of::<WebKitContextMenuItemClass>()}),
    ("WebKitCookieAcceptPolicy", Layout {size: size_of::<WebKitCookieAcceptPolicy>(), alignment: align_of::<WebKitCookieAcceptPolicy>()}),
    ("WebKitCookieManager", Layout {size: size_of::<WebKitCookieManager>(), alignment: align_of::<WebKitCookieManager>()}),
    ("WebKitCookieManagerClass", Layout {size: size_of::<WebKitCookieManagerClass>(), alignment: align_of::<WebKitCookieManagerClass>()}),
    ("WebKitCookiePersistentStorage", Layout {size: size_of::<WebKitCookiePersistentStorage>(), alignment: align_of::<WebKitCookiePersistentStorage>()}),
    ("WebKitCredentialPersistence", Layout {size: size_of::<WebKitCredentialPersistence>(), alignment: align_of::<WebKitCredentialPersistence>()}),
    ("WebKitDownload", Layout {size: size_of::<WebKitDownload>(), alignment: align_of::<WebKitDownload>()}),
    ("WebKitDownloadClass", Layout {size: size_of::<WebKitDownloadClass>(), alignment: align_of::<WebKitDownloadClass>()}),
    ("WebKitDownloadError", Layout {size: size_of::<WebKitDownloadError>(), alignment: align_of::<WebKitDownloadError>()}),
    ("WebKitEditorState", Layout {size: size_of::<WebKitEditorState>(), alignment: align_of::<WebKitEditorState>()}),
    ("WebKitEditorStateClass", Layout {size: size_of::<WebKitEditorStateClass>(), alignment: align_of::<WebKitEditorStateClass>()}),
    ("WebKitEditorTypingAttributes", Layout {size: size_of::<WebKitEditorTypingAttributes>(), alignment: align_of::<WebKitEditorTypingAttributes>()}),
    ("WebKitFaviconDatabase", Layout {size: size_of::<WebKitFaviconDatabase>(), alignment: align_of::<WebKitFaviconDatabase>()}),
    ("WebKitFaviconDatabaseClass", Layout {size: size_of::<WebKitFaviconDatabaseClass>(), alignment: align_of::<WebKitFaviconDatabaseClass>()}),
    ("WebKitFaviconDatabaseError", Layout {size: size_of::<WebKitFaviconDatabaseError>(), alignment: align_of::<WebKitFaviconDatabaseError>()}),
    ("WebKitFileChooserRequest", Layout {size: size_of::<WebKitFileChooserRequest>(), alignment: align_of::<WebKitFileChooserRequest>()}),
    ("WebKitFileChooserRequestClass", Layout {size: size_of::<WebKitFileChooserRequestClass>(), alignment: align_of::<WebKitFileChooserRequestClass>()}),
    ("WebKitFindController", Layout {size: size_of::<WebKitFindController>(), alignment: align_of::<WebKitFindController>()}),
    ("WebKitFindControllerClass", Layout {size: size_of::<WebKitFindControllerClass>(), alignment: align_of::<WebKitFindControllerClass>()}),
    ("WebKitFindOptions", Layout {size: size_of::<WebKitFindOptions>(), alignment: align_of::<WebKitFindOptions>()}),
    ("WebKitFormSubmissionRequest", Layout {size: size_of::<WebKitFormSubmissionRequest>(), alignment: align_of::<WebKitFormSubmissionRequest>()}),
    ("WebKitFormSubmissionRequestClass", Layout {size: size_of::<WebKitFormSubmissionRequestClass>(), alignment: align_of::<WebKitFormSubmissionRequestClass>()}),
    ("WebKitGeolocationPermissionRequest", Layout {size: size_of::<WebKitGeolocationPermissionRequest>(), alignment: align_of::<WebKitGeolocationPermissionRequest>()}),
    ("WebKitGeolocationPermissionRequestClass", Layout {size: size_of::<WebKitGeolocationPermissionRequestClass>(), alignment: align_of::<WebKitGeolocationPermissionRequestClass>()}),
    ("WebKitHardwareAccelerationPolicy", Layout {size: size_of::<WebKitHardwareAccelerationPolicy>(), alignment: align_of::<WebKitHardwareAccelerationPolicy>()}),
    ("WebKitHitTestResult", Layout {size: size_of::<WebKitHitTestResult>(), alignment: align_of::<WebKitHitTestResult>()}),
    ("WebKitHitTestResultClass", Layout {size: size_of::<WebKitHitTestResultClass>(), alignment: align_of::<WebKitHitTestResultClass>()}),
    ("WebKitHitTestResultContext", Layout {size: size_of::<WebKitHitTestResultContext>(), alignment: align_of::<WebKitHitTestResultContext>()}),
    ("WebKitInsecureContentEvent", Layout {size: size_of::<WebKitInsecureContentEvent>(), alignment: align_of::<WebKitInsecureContentEvent>()}),
    ("WebKitInstallMissingMediaPluginsPermissionRequest", Layout {size: size_of::<WebKitInstallMissingMediaPluginsPermissionRequest>(), alignment: align_of::<WebKitInstallMissingMediaPluginsPermissionRequest>()}),
    ("WebKitInstallMissingMediaPluginsPermissionRequestClass", Layout {size: size_of::<WebKitInstallMissingMediaPluginsPermissionRequestClass>(), alignment: align_of::<WebKitInstallMissingMediaPluginsPermissionRequestClass>()}),
    ("WebKitJavascriptError", Layout {size: size_of::<WebKitJavascriptError>(), alignment: align_of::<WebKitJavascriptError>()}),
    ("WebKitLoadEvent", Layout {size: size_of::<WebKitLoadEvent>(), alignment: align_of::<WebKitLoadEvent>()}),
    ("WebKitNavigationPolicyDecision", Layout {size: size_of::<WebKitNavigationPolicyDecision>(), alignment: align_of::<WebKitNavigationPolicyDecision>()}),
    ("WebKitNavigationPolicyDecisionClass", Layout {size: size_of::<WebKitNavigationPolicyDecisionClass>(), alignment: align_of::<WebKitNavigationPolicyDecisionClass>()}),
    ("WebKitNavigationType", Layout {size: size_of::<WebKitNavigationType>(), alignment: align_of::<WebKitNavigationType>()}),
    ("WebKitNetworkError", Layout {size: size_of::<WebKitNetworkError>(), alignment: align_of::<WebKitNetworkError>()}),
    ("WebKitNetworkProxyMode", Layout {size: size_of::<WebKitNetworkProxyMode>(), alignment: align_of::<WebKitNetworkProxyMode>()}),
    ("WebKitNotification", Layout {size: size_of::<WebKitNotification>(), alignment: align_of::<WebKitNotification>()}),
    ("WebKitNotificationClass", Layout {size: size_of::<WebKitNotificationClass>(), alignment: align_of::<WebKitNotificationClass>()}),
    ("WebKitNotificationPermissionRequest", Layout {size: size_of::<WebKitNotificationPermissionRequest>(), alignment: align_of::<WebKitNotificationPermissionRequest>()}),
    ("WebKitNotificationPermissionRequestClass", Layout {size: size_of::<WebKitNotificationPermissionRequestClass>(), alignment: align_of::<WebKitNotificationPermissionRequestClass>()}),
    ("WebKitOptionMenu", Layout {size: size_of::<WebKitOptionMenu>(), alignment: align_of::<WebKitOptionMenu>()}),
    ("WebKitOptionMenuClass", Layout {size: size_of::<WebKitOptionMenuClass>(), alignment: align_of::<WebKitOptionMenuClass>()}),
    ("WebKitPermissionRequestIface", Layout {size: size_of::<WebKitPermissionRequestIface>(), alignment: align_of::<WebKitPermissionRequestIface>()}),
    ("WebKitPlugin", Layout {size: size_of::<WebKitPlugin>(), alignment: align_of::<WebKitPlugin>()}),
    ("WebKitPluginClass", Layout {size: size_of::<WebKitPluginClass>(), alignment: align_of::<WebKitPluginClass>()}),
    ("WebKitPluginError", Layout {size: size_of::<WebKitPluginError>(), alignment: align_of::<WebKitPluginError>()}),
    ("WebKitPolicyDecision", Layout {size: size_of::<WebKitPolicyDecision>(), alignment: align_of::<WebKitPolicyDecision>()}),
    ("WebKitPolicyDecisionClass", Layout {size: size_of::<WebKitPolicyDecisionClass>(), alignment: align_of::<WebKitPolicyDecisionClass>()}),
    ("WebKitPolicyDecisionType", Layout {size: size_of::<WebKitPolicyDecisionType>(), alignment: align_of::<WebKitPolicyDecisionType>()}),
    ("WebKitPolicyError", Layout {size: size_of::<WebKitPolicyError>(), alignment: align_of::<WebKitPolicyError>()}),
    ("WebKitPrintCustomWidget", Layout {size: size_of::<WebKitPrintCustomWidget>(), alignment: align_of::<WebKitPrintCustomWidget>()}),
    ("WebKitPrintCustomWidgetClass", Layout {size: size_of::<WebKitPrintCustomWidgetClass>(), alignment: align_of::<WebKitPrintCustomWidgetClass>()}),
    ("WebKitPrintError", Layout {size: size_of::<WebKitPrintError>(), alignment: align_of::<WebKitPrintError>()}),
    ("WebKitPrintOperation", Layout {size: size_of::<WebKitPrintOperation>(), alignment: align_of::<WebKitPrintOperation>()}),
    ("WebKitPrintOperationClass", Layout {size: size_of::<WebKitPrintOperationClass>(), alignment: align_of::<WebKitPrintOperationClass>()}),
    ("WebKitPrintOperationResponse", Layout {size: size_of::<WebKitPrintOperationResponse>(), alignment: align_of::<WebKitPrintOperationResponse>()}),
    ("WebKitProcessModel", Layout {size: size_of::<WebKitProcessModel>(), alignment: align_of::<WebKitProcessModel>()}),
    ("WebKitResponsePolicyDecision", Layout {size: size_of::<WebKitResponsePolicyDecision>(), alignment: align_of::<WebKitResponsePolicyDecision>()}),
    ("WebKitResponsePolicyDecisionClass", Layout {size: size_of::<WebKitResponsePolicyDecisionClass>(), alignment: align_of::<WebKitResponsePolicyDecisionClass>()}),
    ("WebKitSaveMode", Layout {size: size_of::<WebKitSaveMode>(), alignment: align_of::<WebKitSaveMode>()}),
    ("WebKitScriptDialogType", Layout {size: size_of::<WebKitScriptDialogType>(), alignment: align_of::<WebKitScriptDialogType>()}),
    ("WebKitSecurityManager", Layout {size: size_of::<WebKitSecurityManager>(), alignment: align_of::<WebKitSecurityManager>()}),
    ("WebKitSecurityManagerClass", Layout {size: size_of::<WebKitSecurityManagerClass>(), alignment: align_of::<WebKitSecurityManagerClass>()}),
    ("WebKitSettings", Layout {size: size_of::<WebKitSettings>(), alignment: align_of::<WebKitSettings>()}),
    ("WebKitSettingsClass", Layout {size: size_of::<WebKitSettingsClass>(), alignment: align_of::<WebKitSettingsClass>()}),
    ("WebKitSnapshotError", Layout {size: size_of::<WebKitSnapshotError>(), alignment: align_of::<WebKitSnapshotError>()}),
    ("WebKitSnapshotOptions", Layout {size: size_of::<WebKitSnapshotOptions>(), alignment: align_of::<WebKitSnapshotOptions>()}),
    ("WebKitSnapshotRegion", Layout {size: size_of::<WebKitSnapshotRegion>(), alignment: align_of::<WebKitSnapshotRegion>()}),
    ("WebKitTLSErrorsPolicy", Layout {size: size_of::<WebKitTLSErrorsPolicy>(), alignment: align_of::<WebKitTLSErrorsPolicy>()}),
    ("WebKitURIRequest", Layout {size: size_of::<WebKitURIRequest>(), alignment: align_of::<WebKitURIRequest>()}),
    ("WebKitURIRequestClass", Layout {size: size_of::<WebKitURIRequestClass>(), alignment: align_of::<WebKitURIRequestClass>()}),
    ("WebKitURIResponse", Layout {size: size_of::<WebKitURIResponse>(), alignment: align_of::<WebKitURIResponse>()}),
    ("WebKitURIResponseClass", Layout {size: size_of::<WebKitURIResponseClass>(), alignment: align_of::<WebKitURIResponseClass>()}),
    ("WebKitURISchemeRequest", Layout {size: size_of::<WebKitURISchemeRequest>(), alignment: align_of::<WebKitURISchemeRequest>()}),
    ("WebKitURISchemeRequestClass", Layout {size: size_of::<WebKitURISchemeRequestClass>(), alignment: align_of::<WebKitURISchemeRequestClass>()}),
    ("WebKitUserContentInjectedFrames", Layout {size: size_of::<WebKitUserContentInjectedFrames>(), alignment: align_of::<WebKitUserContentInjectedFrames>()}),
    ("WebKitUserContentManager", Layout {size: size_of::<WebKitUserContentManager>(), alignment: align_of::<WebKitUserContentManager>()}),
    ("WebKitUserContentManagerClass", Layout {size: size_of::<WebKitUserContentManagerClass>(), alignment: align_of::<WebKitUserContentManagerClass>()}),
    ("WebKitUserMediaPermissionRequest", Layout {size: size_of::<WebKitUserMediaPermissionRequest>(), alignment: align_of::<WebKitUserMediaPermissionRequest>()}),
    ("WebKitUserMediaPermissionRequestClass", Layout {size: size_of::<WebKitUserMediaPermissionRequestClass>(), alignment: align_of::<WebKitUserMediaPermissionRequestClass>()}),
    ("WebKitUserScriptInjectionTime", Layout {size: size_of::<WebKitUserScriptInjectionTime>(), alignment: align_of::<WebKitUserScriptInjectionTime>()}),
    ("WebKitUserStyleLevel", Layout {size: size_of::<WebKitUserStyleLevel>(), alignment: align_of::<WebKitUserStyleLevel>()}),
    ("WebKitWebContext", Layout {size: size_of::<WebKitWebContext>(), alignment: align_of::<WebKitWebContext>()}),
    ("WebKitWebContextClass", Layout {size: size_of::<WebKitWebContextClass>(), alignment: align_of::<WebKitWebContextClass>()}),
    ("WebKitWebInspector", Layout {size: size_of::<WebKitWebInspector>(), alignment: align_of::<WebKitWebInspector>()}),
    ("WebKitWebInspectorClass", Layout {size: size_of::<WebKitWebInspectorClass>(), alignment: align_of::<WebKitWebInspectorClass>()}),
    ("WebKitWebProcessTerminationReason", Layout {size: size_of::<WebKitWebProcessTerminationReason>(), alignment: align_of::<WebKitWebProcessTerminationReason>()}),
    ("WebKitWebResource", Layout {size: size_of::<WebKitWebResource>(), alignment: align_of::<WebKitWebResource>()}),
    ("WebKitWebResourceClass", Layout {size: size_of::<WebKitWebResourceClass>(), alignment: align_of::<WebKitWebResourceClass>()}),
    ("WebKitWebView", Layout {size: size_of::<WebKitWebView>(), alignment: align_of::<WebKitWebView>()}),
    ("WebKitWebViewBase", Layout {size: size_of::<WebKitWebViewBase>(), alignment: align_of::<WebKitWebViewBase>()}),
    ("WebKitWebsiteDataManager", Layout {size: size_of::<WebKitWebsiteDataManager>(), alignment: align_of::<WebKitWebsiteDataManager>()}),
    ("WebKitWebsiteDataManagerClass", Layout {size: size_of::<WebKitWebsiteDataManagerClass>(), alignment: align_of::<WebKitWebsiteDataManagerClass>()}),
    ("WebKitWebsiteDataTypes", Layout {size: size_of::<WebKitWebsiteDataTypes>(), alignment: align_of::<WebKitWebsiteDataTypes>()}),
    ("WebKitWindowProperties", Layout {size: size_of::<WebKitWindowProperties>(), alignment: align_of::<WebKitWindowProperties>()}),
    ("WebKitWindowPropertiesClass", Layout {size: size_of::<WebKitWindowPropertiesClass>(), alignment: align_of::<WebKitWindowPropertiesClass>()}),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("WEBKIT_AUTHENTICATION_SCHEME_CLIENT_CERTIFICATE_REQUESTED", "7"),
    ("WEBKIT_AUTHENTICATION_SCHEME_DEFAULT", "1"),
    ("WEBKIT_AUTHENTICATION_SCHEME_HTML_FORM", "4"),
    ("WEBKIT_AUTHENTICATION_SCHEME_HTTP_BASIC", "2"),
    ("WEBKIT_AUTHENTICATION_SCHEME_HTTP_DIGEST", "3"),
    ("WEBKIT_AUTHENTICATION_SCHEME_NEGOTIATE", "6"),
    ("WEBKIT_AUTHENTICATION_SCHEME_NTLM", "5"),
    ("WEBKIT_AUTHENTICATION_SCHEME_SERVER_TRUST_EVALUATION_REQUESTED", "8"),
    ("WEBKIT_AUTHENTICATION_SCHEME_UNKNOWN", "100"),
    ("WEBKIT_CACHE_MODEL_DOCUMENT_BROWSER", "2"),
    ("WEBKIT_CACHE_MODEL_DOCUMENT_VIEWER", "0"),
    ("WEBKIT_CACHE_MODEL_WEB_BROWSER", "1"),
    ("WEBKIT_CONTEXT_MENU_ACTION_BOLD", "27"),
    ("WEBKIT_CONTEXT_MENU_ACTION_COPY", "14"),
    ("WEBKIT_CONTEXT_MENU_ACTION_COPY_AUDIO_LINK_TO_CLIPBOARD", "35"),
    ("WEBKIT_CONTEXT_MENU_ACTION_COPY_IMAGE_TO_CLIPBOARD", "7"),
    ("WEBKIT_CONTEXT_MENU_ACTION_COPY_IMAGE_URL_TO_CLIPBOARD", "8"),
    ("WEBKIT_CONTEXT_MENU_ACTION_COPY_LINK_TO_CLIPBOARD", "4"),
    ("WEBKIT_CONTEXT_MENU_ACTION_COPY_VIDEO_LINK_TO_CLIPBOARD", "34"),
    ("WEBKIT_CONTEXT_MENU_ACTION_CUSTOM", "10000"),
    ("WEBKIT_CONTEXT_MENU_ACTION_CUT", "15"),
    ("WEBKIT_CONTEXT_MENU_ACTION_DELETE", "17"),
    ("WEBKIT_CONTEXT_MENU_ACTION_DOWNLOAD_AUDIO_TO_DISK", "43"),
    ("WEBKIT_CONTEXT_MENU_ACTION_DOWNLOAD_IMAGE_TO_DISK", "6"),
    ("WEBKIT_CONTEXT_MENU_ACTION_DOWNLOAD_LINK_TO_DISK", "3"),
    ("WEBKIT_CONTEXT_MENU_ACTION_DOWNLOAD_VIDEO_TO_DISK", "42"),
    ("WEBKIT_CONTEXT_MENU_ACTION_ENTER_VIDEO_FULLSCREEN", "38"),
    ("WEBKIT_CONTEXT_MENU_ACTION_FONT_MENU", "26"),
    ("WEBKIT_CONTEXT_MENU_ACTION_GO_BACK", "10"),
    ("WEBKIT_CONTEXT_MENU_ACTION_GO_FORWARD", "11"),
    ("WEBKIT_CONTEXT_MENU_ACTION_IGNORE_GRAMMAR", "25"),
    ("WEBKIT_CONTEXT_MENU_ACTION_IGNORE_SPELLING", "23"),
    ("WEBKIT_CONTEXT_MENU_ACTION_INPUT_METHODS", "19"),
    ("WEBKIT_CONTEXT_MENU_ACTION_INSPECT_ELEMENT", "31"),
    ("WEBKIT_CONTEXT_MENU_ACTION_ITALIC", "28"),
    ("WEBKIT_CONTEXT_MENU_ACTION_LEARN_SPELLING", "24"),
    ("WEBKIT_CONTEXT_MENU_ACTION_MEDIA_MUTE", "41"),
    ("WEBKIT_CONTEXT_MENU_ACTION_MEDIA_PAUSE", "40"),
    ("WEBKIT_CONTEXT_MENU_ACTION_MEDIA_PLAY", "39"),
    ("WEBKIT_CONTEXT_MENU_ACTION_NO_ACTION", "0"),
    ("WEBKIT_CONTEXT_MENU_ACTION_NO_GUESSES_FOUND", "22"),
    ("WEBKIT_CONTEXT_MENU_ACTION_OPEN_AUDIO_IN_NEW_WINDOW", "33"),
    ("WEBKIT_CONTEXT_MENU_ACTION_OPEN_FRAME_IN_NEW_WINDOW", "9"),
    ("WEBKIT_CONTEXT_MENU_ACTION_OPEN_IMAGE_IN_NEW_WINDOW", "5"),
    ("WEBKIT_CONTEXT_MENU_ACTION_OPEN_LINK", "1"),
    ("WEBKIT_CONTEXT_MENU_ACTION_OPEN_LINK_IN_NEW_WINDOW", "2"),
    ("WEBKIT_CONTEXT_MENU_ACTION_OPEN_VIDEO_IN_NEW_WINDOW", "32"),
    ("WEBKIT_CONTEXT_MENU_ACTION_OUTLINE", "30"),
    ("WEBKIT_CONTEXT_MENU_ACTION_PASTE", "16"),
    ("WEBKIT_CONTEXT_MENU_ACTION_RELOAD", "13"),
    ("WEBKIT_CONTEXT_MENU_ACTION_SELECT_ALL", "18"),
    ("WEBKIT_CONTEXT_MENU_ACTION_SPELLING_GUESS", "21"),
    ("WEBKIT_CONTEXT_MENU_ACTION_STOP", "12"),
    ("WEBKIT_CONTEXT_MENU_ACTION_TOGGLE_MEDIA_CONTROLS", "36"),
    ("WEBKIT_CONTEXT_MENU_ACTION_TOGGLE_MEDIA_LOOP", "37"),
    ("WEBKIT_CONTEXT_MENU_ACTION_UNDERLINE", "29"),
    ("WEBKIT_CONTEXT_MENU_ACTION_UNICODE", "20"),
    ("WEBKIT_COOKIE_PERSISTENT_STORAGE_SQLITE", "1"),
    ("WEBKIT_COOKIE_PERSISTENT_STORAGE_TEXT", "0"),
    ("WEBKIT_COOKIE_POLICY_ACCEPT_ALWAYS", "0"),
    ("WEBKIT_COOKIE_POLICY_ACCEPT_NEVER", "1"),
    ("WEBKIT_COOKIE_POLICY_ACCEPT_NO_THIRD_PARTY", "2"),
    ("WEBKIT_CREDENTIAL_PERSISTENCE_FOR_SESSION", "1"),
    ("WEBKIT_CREDENTIAL_PERSISTENCE_NONE", "0"),
    ("WEBKIT_CREDENTIAL_PERSISTENCE_PERMANENT", "2"),
    ("WEBKIT_DOWNLOAD_ERROR_CANCELLED_BY_USER", "400"),
    ("WEBKIT_DOWNLOAD_ERROR_DESTINATION", "401"),
    ("WEBKIT_DOWNLOAD_ERROR_NETWORK", "499"),
    ("WEBKIT_EDITING_COMMAND_COPY", "Copy"),
    ("WEBKIT_EDITING_COMMAND_CREATE_LINK", "CreateLink"),
    ("WEBKIT_EDITING_COMMAND_CUT", "Cut"),
    ("WEBKIT_EDITING_COMMAND_INSERT_IMAGE", "InsertImage"),
    ("WEBKIT_EDITING_COMMAND_PASTE", "Paste"),
    ("WEBKIT_EDITING_COMMAND_REDO", "Redo"),
    ("WEBKIT_EDITING_COMMAND_SELECT_ALL", "SelectAll"),
    ("WEBKIT_EDITING_COMMAND_UNDO", "Undo"),
    ("WEBKIT_EDITOR_TYPING_ATTRIBUTE_BOLD", "4"),
    ("WEBKIT_EDITOR_TYPING_ATTRIBUTE_ITALIC", "8"),
    ("WEBKIT_EDITOR_TYPING_ATTRIBUTE_NONE", "2"),
    ("WEBKIT_EDITOR_TYPING_ATTRIBUTE_STRIKETHROUGH", "32"),
    ("WEBKIT_EDITOR_TYPING_ATTRIBUTE_UNDERLINE", "16"),
    ("WEBKIT_FAVICON_DATABASE_ERROR_FAVICON_NOT_FOUND", "1"),
    ("WEBKIT_FAVICON_DATABASE_ERROR_FAVICON_UNKNOWN", "2"),
    ("WEBKIT_FAVICON_DATABASE_ERROR_NOT_INITIALIZED", "0"),
    ("WEBKIT_FIND_OPTIONS_AT_WORD_STARTS", "2"),
    ("WEBKIT_FIND_OPTIONS_BACKWARDS", "8"),
    ("WEBKIT_FIND_OPTIONS_CASE_INSENSITIVE", "1"),
    ("WEBKIT_FIND_OPTIONS_NONE", "0"),
    ("WEBKIT_FIND_OPTIONS_TREAT_MEDIAL_CAPITAL_AS_WORD_START", "4"),
    ("WEBKIT_FIND_OPTIONS_WRAP_AROUND", "16"),
    ("WEBKIT_HARDWARE_ACCELERATION_POLICY_ALWAYS", "1"),
    ("WEBKIT_HARDWARE_ACCELERATION_POLICY_NEVER", "2"),
    ("WEBKIT_HARDWARE_ACCELERATION_POLICY_ON_DEMAND", "0"),
    ("WEBKIT_HIT_TEST_RESULT_CONTEXT_DOCUMENT", "2"),
    ("WEBKIT_HIT_TEST_RESULT_CONTEXT_EDITABLE", "32"),
    ("WEBKIT_HIT_TEST_RESULT_CONTEXT_IMAGE", "8"),
    ("WEBKIT_HIT_TEST_RESULT_CONTEXT_LINK", "4"),
    ("WEBKIT_HIT_TEST_RESULT_CONTEXT_MEDIA", "16"),
    ("WEBKIT_HIT_TEST_RESULT_CONTEXT_SCROLLBAR", "64"),
    ("WEBKIT_HIT_TEST_RESULT_CONTEXT_SELECTION", "128"),
    ("WEBKIT_INSECURE_CONTENT_DISPLAYED", "1"),
    ("WEBKIT_INSECURE_CONTENT_RUN", "0"),
    ("WEBKIT_JAVASCRIPT_ERROR_SCRIPT_FAILED", "699"),
    ("WEBKIT_LOAD_COMMITTED", "2"),
    ("WEBKIT_LOAD_FINISHED", "3"),
    ("WEBKIT_LOAD_REDIRECTED", "1"),
    ("WEBKIT_LOAD_STARTED", "0"),
    ("WEBKIT_MAJOR_VERSION", "2"),
    ("WEBKIT_MICRO_VERSION", "2"),
    ("WEBKIT_MINOR_VERSION", "20"),
    ("WEBKIT_NAVIGATION_TYPE_BACK_FORWARD", "2"),
    ("WEBKIT_NAVIGATION_TYPE_FORM_RESUBMITTED", "4"),
    ("WEBKIT_NAVIGATION_TYPE_FORM_SUBMITTED", "1"),
    ("WEBKIT_NAVIGATION_TYPE_LINK_CLICKED", "0"),
    ("WEBKIT_NAVIGATION_TYPE_OTHER", "5"),
    ("WEBKIT_NAVIGATION_TYPE_RELOAD", "3"),
    ("WEBKIT_NETWORK_ERROR_CANCELLED", "302"),
    ("WEBKIT_NETWORK_ERROR_FAILED", "399"),
    ("WEBKIT_NETWORK_ERROR_FILE_DOES_NOT_EXIST", "303"),
    ("WEBKIT_NETWORK_ERROR_TRANSPORT", "300"),
    ("WEBKIT_NETWORK_ERROR_UNKNOWN_PROTOCOL", "301"),
    ("WEBKIT_NETWORK_PROXY_MODE_CUSTOM", "2"),
    ("WEBKIT_NETWORK_PROXY_MODE_DEFAULT", "0"),
    ("WEBKIT_NETWORK_PROXY_MODE_NO_PROXY", "1"),
    ("WEBKIT_PLUGIN_ERROR_CANNOT_FIND_PLUGIN", "200"),
    ("WEBKIT_PLUGIN_ERROR_CANNOT_LOAD_PLUGIN", "201"),
    ("WEBKIT_PLUGIN_ERROR_CONNECTION_CANCELLED", "203"),
    ("WEBKIT_PLUGIN_ERROR_FAILED", "299"),
    ("WEBKIT_PLUGIN_ERROR_JAVA_UNAVAILABLE", "202"),
    ("WEBKIT_PLUGIN_ERROR_WILL_HANDLE_LOAD", "204"),
    ("WEBKIT_POLICY_DECISION_TYPE_NAVIGATION_ACTION", "0"),
    ("WEBKIT_POLICY_DECISION_TYPE_NEW_WINDOW_ACTION", "1"),
    ("WEBKIT_POLICY_DECISION_TYPE_RESPONSE", "2"),
    ("WEBKIT_POLICY_ERROR_CANNOT_SHOW_MIME_TYPE", "100"),
    ("WEBKIT_POLICY_ERROR_CANNOT_SHOW_URI", "101"),
    ("WEBKIT_POLICY_ERROR_CANNOT_USE_RESTRICTED_PORT", "103"),
    ("WEBKIT_POLICY_ERROR_FAILED", "199"),
    ("WEBKIT_POLICY_ERROR_FRAME_LOAD_INTERRUPTED_BY_POLICY_CHANGE", "102"),
    ("WEBKIT_PRINT_ERROR_GENERAL", "599"),
    ("WEBKIT_PRINT_ERROR_INVALID_PAGE_RANGE", "501"),
    ("WEBKIT_PRINT_ERROR_PRINTER_NOT_FOUND", "500"),
    ("WEBKIT_PRINT_OPERATION_RESPONSE_CANCEL", "1"),
    ("WEBKIT_PRINT_OPERATION_RESPONSE_PRINT", "0"),
    ("WEBKIT_PROCESS_MODEL_MULTIPLE_SECONDARY_PROCESSES", "1"),
    ("WEBKIT_PROCESS_MODEL_SHARED_SECONDARY_PROCESS", "0"),
    ("WEBKIT_SAVE_MODE_MHTML", "0"),
    ("WEBKIT_SCRIPT_DIALOG_ALERT", "0"),
    ("WEBKIT_SCRIPT_DIALOG_BEFORE_UNLOAD_CONFIRM", "3"),
    ("WEBKIT_SCRIPT_DIALOG_CONFIRM", "1"),
    ("WEBKIT_SCRIPT_DIALOG_PROMPT", "2"),
    ("WEBKIT_SNAPSHOT_ERROR_FAILED_TO_CREATE", "799"),
    ("WEBKIT_SNAPSHOT_OPTIONS_INCLUDE_SELECTION_HIGHLIGHTING", "1"),
    ("WEBKIT_SNAPSHOT_OPTIONS_NONE", "0"),
    ("WEBKIT_SNAPSHOT_OPTIONS_TRANSPARENT_BACKGROUND", "2"),
    ("WEBKIT_SNAPSHOT_REGION_FULL_DOCUMENT", "1"),
    ("WEBKIT_SNAPSHOT_REGION_VISIBLE", "0"),
    ("WEBKIT_TLS_ERRORS_POLICY_FAIL", "1"),
    ("WEBKIT_TLS_ERRORS_POLICY_IGNORE", "0"),
    ("WEBKIT_USER_CONTENT_INJECT_ALL_FRAMES", "0"),
    ("WEBKIT_USER_CONTENT_INJECT_TOP_FRAME", "1"),
    ("WEBKIT_USER_SCRIPT_INJECT_AT_DOCUMENT_END", "1"),
    ("WEBKIT_USER_SCRIPT_INJECT_AT_DOCUMENT_START", "0"),
    ("WEBKIT_USER_STYLE_LEVEL_AUTHOR", "1"),
    ("WEBKIT_USER_STYLE_LEVEL_USER", "0"),
    ("WEBKIT_WEBSITE_DATA_ALL", "511"),
    ("WEBKIT_WEBSITE_DATA_COOKIES", "256"),
    ("WEBKIT_WEBSITE_DATA_DISK_CACHE", "2"),
    ("WEBKIT_WEBSITE_DATA_INDEXEDDB_DATABASES", "64"),
    ("WEBKIT_WEBSITE_DATA_LOCAL_STORAGE", "16"),
    ("WEBKIT_WEBSITE_DATA_MEMORY_CACHE", "1"),
    ("WEBKIT_WEBSITE_DATA_OFFLINE_APPLICATION_CACHE", "4"),
    ("WEBKIT_WEBSITE_DATA_PLUGIN_DATA", "128"),
    ("WEBKIT_WEBSITE_DATA_SESSION_STORAGE", "8"),
    ("WEBKIT_WEBSITE_DATA_WEBSQL_DATABASES", "32"),
    ("WEBKIT_WEB_PROCESS_CRASHED", "0"),
    ("WEBKIT_WEB_PROCESS_EXCEEDED_MEMORY_LIMIT", "1"),
];


