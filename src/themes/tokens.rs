//! Material Design 3 CSS Token constants
//!
//! These tokens are used to generate CSS variables for theming.
//! Based on Material Design 3 token specification.

/// Light theme CSS variables
pub const LIGHT_TOKENS: &str = r#"
--md-primary: #6366f1;
--md-on-primary: #ffffff;
--md-primary-container: #e0e7ff;
--md-on-primary-container: #1e1b4b;
--md-secondary: #625b71;
--md-on-secondary: #ffffff;
--md-secondary-container: #e8def8;
--md-on-secondary-container: #1d192b;
--md-tertiary: #7d5260;
--md-on-tertiary: #ffffff;
--md-tertiary-container: #ffd8e4;
--md-on-tertiary-container: #31111d;
--md-error: #b3261e;
--md-on-error: #ffffff;
--md-error-container: #f9dedc;
--md-on-error-container: #410e0b;
--md-background: #fefbff;
--md-on-background: #1c1b1f;
--md-surface: #fefbff;
--md-on-surface: #1c1b1f;
--md-surface-variant: #e7e0ec;
--md-on-surface-variant: #49454f;
--md-outline: #79747e;
--md-outline-variant: #cac4d0;
--md-shadow: #000000;
--md-scrim: #000000;
--md-inverse-surface: #313033;
--md-inverse-on-surface: #f4eff4;
--md-inverse-primary: #a5b4fc;
--md-surface-dim: #ded8e1;
--md-surface-bright: #fefbff;
--md-surface-container-lowest: #ffffff;
--md-surface-container-low: #f7f2fa;
--md-surface-container: #f3edf7;
--md-surface-container-high: #ede7f2;
--md-surface-container-highest: #e8e0ec;
--md-surface-disabled: rgba(28, 27, 31, 0.12);
--md-on-surface-disabled: rgba(28, 27, 31, 0.38);
--md-outline-style: solid;
--md-outline-width: 1px;
"#;

/// Dark theme CSS variables
pub const DARK_TOKENS: &str = r#"
--md-primary: #a5b4fc;
--md-on-primary: #312e81;
--md-primary-container: #4338ca;
--md-on-primary-container: #e0e7ff;
--md-secondary: #cbc2db;
--md-on-secondary: #332d41;
--md-secondary-container: #4a4458;
--md-on-secondary-container: #e8def8;
--md-tertiary: #efb8c8;
--md-on-tertiary: #492532;
--md-tertiary-container: #633b48;
--md-on-tertiary-container: #ffd8e4;
--md-error: #f2b8b5;
--md-on-error: #601410;
--md-error-container: #8c1d18;
--md-on-error-container: #f9dedc;
--md-background: #1c1b1f;
--md-on-background: #e6e1e5;
--md-surface: #1c1b1f;
--md-on-surface: #e6e1e5;
--md-surface-variant: #49454f;
--md-on-surface-variant: #cac4d0;
--md-outline: #938f99;
--md-outline-variant: #49454f;
--md-shadow: #000000;
--md-scrim: #000000;
--md-inverse-surface: #e6e1e5;
--md-inverse-on-surface: #313033;
--md-inverse-primary: #6366f1;
--md-surface-dim: #141218;
--md-surface-bright: #3b383e;
--md-surface-container-lowest: #0f0d13;
--md-surface-container-low: #1d1b20;
--md-surface-container: #211f26;
--md-surface-container-high: #2b2930;
--md-surface-container-highest: #36343b;
--md-surface-disabled: rgba(230, 225, 229, 0.12);
--md-on-surface-disabled: rgba(230, 225, 229, 0.38);
--md-outline-style: solid;
--md-outline-width: 1px;
"#;

/// Flowbite/TailwindCSS utility classes for common components
pub mod flowbite_classes {
    /// Button variants
    pub mod button {
        /// Primary button classes
        pub const PRIMARY: &str = "text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800";
        
        /// Secondary button classes
        pub const SECONDARY: &str = "text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-gray-800 dark:text-white dark:border-gray-600 dark:hover:bg-gray-700 dark:hover:border-gray-600 dark:focus:ring-gray-700";
        
        /// Outlined button classes
        pub const OUTLINED: &str = "text-blue-700 hover:text-white border border-blue-700 hover:bg-blue-700 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:text-blue-500 dark:border-blue-500 dark:hover:text-white dark:hover:bg-blue-600 dark:focus:ring-blue-800";
        
        /// Text button classes
        pub const TEXT: &str = "text-blue-700 hover:bg-blue-700 hover:text-white focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:text-blue-500 dark:hover:bg-blue-600 dark:hover:text-white dark:focus:ring-blue-800";
        
        /// Tonal button classes
        pub const TONAL: &str = "text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800";
        
        /// Danger button classes
        pub const DANGER: &str = "text-white bg-red-700 hover:bg-red-800 focus:ring-4 focus:ring-red-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-red-600 dark:hover:bg-red-700 dark:focus:ring-red-800";
    }
    
    /// Input classes
    pub mod input {
        pub const DEFAULT: &str = "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500";
        pub const WITH_LABEL: &str = "block mb-2 text-sm font-medium text-gray-900 dark:text-white";
    }
    
    /// Card classes
    pub mod card {
        pub const DEFAULT: &str = "max-w-sm bg-white rounded-lg shadow dark:bg-gray-800 border border-gray-200 dark:border-gray-700";
        pub const ELEVATED: &str = "max-w-sm bg-white rounded-xl shadow-lg dark:bg-gray-800";
        pub const BORDERED: &str = "max-w-sm bg-white rounded-lg border border-gray-200 dark:bg-gray-800 dark:border-gray-700";
    }
    
    /// Modal classes
    pub mod modal {
        pub const OVERLAY: &str = "fixed inset-0 z-50 bg-black bg-opacity-50 flex items-center justify-center";
        pub const CONTENT: &str = "relative bg-white rounded-lg shadow dark:bg-gray-800 max-h-[90vh] overflow-y-auto";
        pub const HEADER: &str = "flex items-center justify-between p-4 md:p-5 border-b rounded-t dark:border-gray-600";
        pub const BODY: &str = "p-4 md:p-5";
        pub const FOOTER: &str = "flex items-center justify-end p-4 md:p-5 border-t border-gray-200 rounded-b dark:border-gray-600";
    }
    
    /// Badge classes
    pub mod badge {
        pub const DEFAULT: &str = "bg-blue-100 text-blue-800 text-xs font-medium px-2.5 py-0.5 rounded dark:bg-blue-900 dark:text-blue-300";
        pub const SUCCESS: &str = "bg-green-100 text-green-800 text-xs font-medium px-2.5 py-0.5 rounded dark:bg-green-900 dark:text-green-300";
        pub const WARNING: &str = "bg-yellow-100 text-yellow-800 text-xs font-medium px-2.5 py-0.5 rounded dark:bg-yellow-900 dark:text-yellow-300";
        pub const ERROR: &str = "bg-red-100 text-red-800 text-xs font-medium px-2.5 py-0.5 rounded dark:bg-red-900 dark:text-red-300";
    }
    
    /// Toast classes
    pub mod toast {
        pub const CONTAINER: &str = "fixed bottom-5 right-5 z-50 flex flex-col gap-2";
        pub const ITEM: &str = "flex items-center w-full max-w-xs p-4 text-gray-500 bg-white rounded-lg shadow dark:text-gray-400 dark:bg-gray-800 border border-gray-200 dark:border-gray-700";
    }
}
