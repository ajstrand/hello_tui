use fluent::{FluentBundle, FluentResource};
use fluent_bundle::{FluentValue, FluentArgs};
use unic_langid::{langid, LanguageIdentifier};
use std::collections::HashMap;
use std::cell::RefCell;

/// Global internationalization manager for the text editor
pub struct I18nManager {
    bundles: HashMap<String, FluentBundle<FluentResource>>,
    current_locale: LanguageIdentifier,
    fallback_locale: LanguageIdentifier,
}

impl I18nManager {
    /// Creates a new I18nManager with default English locale
    pub fn new() -> Self {
        let mut manager = Self {
            bundles: HashMap::new(),
            current_locale: langid!("en-US"),
            fallback_locale: langid!("en-US"),
        };
        
        // Initialize with supported languages
        manager.load_english();
        manager.load_spanish();
        manager.load_french();
        manager.load_german();
        
        manager
    }
    
    /// Sets the current locale
    pub fn set_locale(&mut self, locale: &str) -> Result<(), String> {
        let locale_id: LanguageIdentifier = locale.parse()
            .map_err(|_| format!("Invalid locale: {}", locale))?;
        
        if self.bundles.contains_key(&locale_id.to_string()) {
            self.current_locale = locale_id;
            Ok(())
        } else {
            Err(format!("Locale '{}' not supported", locale))
        }
    }
    
    /// Gets the current locale
    pub fn get_locale(&self) -> &LanguageIdentifier {
        &self.current_locale
    }
    
    /// Gets a localized message by key
    pub fn get_message(&self, key: &str) -> String {
        self.get_message_with_args(key, None)
    }
    
    /// Gets a localized message by key with arguments
    pub fn get_message_with_args(&self, key: &str, args: Option<HashMap<String, FluentValue>>) -> String {
        let locale_str = self.current_locale.to_string();
        
        // Try current locale
        if let Some(bundle) = self.bundles.get(&locale_str) {
            if let Some(msg) = bundle.get_message(key) {
                if let Some(pattern) = msg.value() {
                    let mut errors = Vec::new();
                    let fluent_args = args.as_ref().map(|a| {
                        let mut fa = FluentArgs::new();
                        for (k, v) in a.iter() {
                            fa.set(k, v.clone());
                        }
                        fa
                    });
                    return bundle.format_pattern(pattern, fluent_args.as_ref(), &mut errors).to_string();
                }
            }
        }
        
        // Fallback to English
        let fallback_str = self.fallback_locale.to_string();
        if let Some(bundle) = self.bundles.get(&fallback_str) {
            if let Some(msg) = bundle.get_message(key) {
                if let Some(pattern) = msg.value() {
                    let mut errors = Vec::new();
                    let fluent_args = args.as_ref().map(|a| {
                        let mut fa = FluentArgs::new();
                        for (k, v) in a.iter() {
                            fa.set(k, v.clone());
                        }
                        fa
                    });
                    return bundle.format_pattern(pattern, fluent_args.as_ref(), &mut errors).to_string();
                }
            }
        }
        
        // Final fallback to key itself
        format!("[{}]", key)
    }
    
    /// Loads English translations
    fn load_english(&mut self) {
        let ftl_string = include_str!("../locales/en-US.ftl");
        let resource = FluentResource::try_new(ftl_string.to_string())
            .expect("Failed to parse English FTL");
        
        let mut bundle = FluentBundle::new(vec![langid!("en-US")]);
        bundle.add_resource(resource).expect("Failed to add English resource");
        
        self.bundles.insert("en-US".to_string(), bundle);
    }
    
    /// Loads Spanish translations
    fn load_spanish(&mut self) {
        let ftl_string = include_str!("../locales/es-ES.ftl");
        let resource = FluentResource::try_new(ftl_string.to_string())
            .expect("Failed to parse Spanish FTL");
        
        let mut bundle = FluentBundle::new(vec![langid!("es-ES")]);
        bundle.add_resource(resource).expect("Failed to add Spanish resource");
        
        self.bundles.insert("es-ES".to_string(), bundle);
    }
    
    /// Loads French translations
    fn load_french(&mut self) {
        let ftl_string = include_str!("../locales/fr-FR.ftl");
        let resource = FluentResource::try_new(ftl_string.to_string())
            .expect("Failed to parse French FTL");
        
        let mut bundle = FluentBundle::new(vec![langid!("fr-FR")]);
        bundle.add_resource(resource).expect("Failed to add French resource");
        
        self.bundles.insert("fr-FR".to_string(), bundle);
    }
    
    /// Loads German translations
    fn load_german(&mut self) {
        let ftl_string = include_str!("../locales/de-DE.ftl");
        let resource = FluentResource::try_new(ftl_string.to_string())
            .expect("Failed to parse German FTL");
        
        let mut bundle = FluentBundle::new(vec![langid!("de-DE")]);
        bundle.add_resource(resource).expect("Failed to add German resource");
        
        self.bundles.insert("de-DE".to_string(), bundle);
    }
    
    /// Gets list of supported locales
    pub fn get_supported_locales(&self) -> Vec<&String> {
        self.bundles.keys().collect()
    }
}

thread_local! {
    static I18N_MANAGER: RefCell<I18nManager> = RefCell::new(I18nManager::new());
}

/// Access the I18nManager for reading
pub fn with_i18n<T, F>(f: F) -> T 
where 
    F: FnOnce(&I18nManager) -> T 
{
    I18N_MANAGER.with(|m| f(&*m.borrow()))
}

/// Access the I18nManager for writing
pub fn with_i18n_mut<T, F>(f: F) -> T 
where 
    F: FnOnce(&mut I18nManager) -> T 
{
    I18N_MANAGER.with(|m| f(&mut *m.borrow_mut()))
}

/// Convenience function to get a localized message
pub fn t(key: &str) -> String {
    with_i18n(|manager| manager.get_message(key))
}

/// Convenience function to get a localized message with arguments
pub fn t_with_args(key: &str, args: HashMap<String, FluentValue>) -> String {
    with_i18n(|manager| manager.get_message_with_args(key, Some(args)))
}

/// Macro for easier translation with arguments
#[macro_export]
macro_rules! t {
    ($key:expr) => {
        crate::i18n::t($key)
    };
    ($key:expr, $($arg_name:ident = $arg_value:expr),+) => {
        {
            let mut args = std::collections::HashMap::new();
            $(
                args.insert(
                    stringify!($arg_name).to_string(),
                    fluent_bundle::FluentValue::from($arg_value)
                );
            )+
            crate::i18n::t_with_args($key, args)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_translation() {
        let manager = I18nManager::new();
        let welcome = manager.get_message("welcome-title");
        assert!(!welcome.is_empty());
        assert!(!welcome.starts_with('['));
    }
    
    #[test]
    fn test_locale_switching() {
        let mut manager = I18nManager::new();
        
        // Test English (default)
        let en_message = manager.get_message("welcome-title");
        
        // Switch to Spanish
        manager.set_locale("es-ES").unwrap();
        let es_message = manager.get_message("welcome-title");
        
        assert_ne!(en_message, es_message);
    }
    
    #[test]
    fn test_translation_with_args() {
        let manager = I18nManager::new();
        let mut args = HashMap::new();
        args.insert("filename".to_string(), FluentValue::from("test.txt"));
        
        let message = manager.get_message_with_args("file-loaded", Some(args));
        assert!(message.contains("test.txt"));
    }
}
