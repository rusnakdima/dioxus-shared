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
        pub const PRIMARY: &str = "focus:ring-4 focus:ring-blue-300 font-medium text-sm px-5 py-2.5 me-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800";
        pub const SECONDARY: &str = "border focus:outline-none focus:ring-4 focus:ring-gray-100 font-medium text-sm px-5 py-2.5 me-2 mb-2 dark:bg-gray-800 dark:border-gray-600 dark:hover:bg-gray-700 dark:hover:border-gray-600 dark:focus:ring-gray-700";
        pub const OUTLINED: &str = "border focus:ring-4 focus:ring-blue-300 font-medium text-sm px-5 py-2.5 me-2 mb-2 dark:text-blue-500 dark:border-blue-500 dark:hover:text-white dark:hover:bg-blue-600 dark:focus:ring-blue-800";
        pub const TEXT: &str = "focus:ring-4 focus:ring-blue-300 font-medium text-sm px-5 py-2.5 me-2 mb-2 dark:text-blue-500 dark:hover:bg-blue-600 dark:hover:text-white dark:focus:ring-blue-800";
        pub const TONAL: &str = "focus:ring-4 focus:ring-blue-300 font-medium text-sm px-5 py-2.5 me-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800";
        pub const DANGER: &str = "text-white bg-red-600 hover:bg-red-700 focus:ring-4 focus:ring-red-300 font-medium text-sm px-5 py-2.5 me-2 mb-2 dark:bg-red-700 dark:hover:bg-red-800 dark:focus:ring-red-800";
        // Size variants
        pub const SM: &str = "text-xs px-3 py-1.5";
        pub const LG: &str = "text-base px-7 py-3";
    }

    pub mod input {
        pub const DEFAULT: &str = "text-sm rounded-lg block w-full p-2.5 ";
        pub const WITH_LABEL: &str = "block mb-2 text-sm font-medium ";
    }

    pub mod card {
        pub const DEFAULT: &str = "max-w-sm ";
        pub const ELEVATED: &str = "max-w-sm ";
        pub const BORDERED: &str = "max-w-sm ";
    }

    pub mod modal {
        pub const OVERLAY: &str = "fixed inset-0 z-50 opacity-50 flex items-center justify-center";
        pub const CONTENT: &str = "relative rounded-lg shadow max-h-[90vh] overflow-y-auto";
        pub const HEADER: &str = "flex items-center justify-between p-4 md:p-5 border-b rounded-t ";
        pub const BODY: &str = "p-4 md:p-5";
        pub const FOOTER: &str = "flex items-center justify-between p-4 md:p-5 border-t rounded-b ";
    }

    pub mod badge {
        pub const DEFAULT: &str = "text-xs font-medium px-2.5 py-0.5 rounded ";
        pub const SUCCESS: &str = "text-xs font-medium px-2.5 py-0.5 rounded bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200";
        pub const WARNING: &str = "text-xs font-medium px-2.5 py-0.5 rounded bg-amber-100 text-amber-800 dark:bg-amber-900 dark:text-amber-200";
        pub const ERROR: &str = "text-xs font-medium px-2.5 py-0.5 rounded bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200";
        pub const INFO: &str = "text-xs font-medium px-2.5 py-0.5 rounded bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200";
    }

    pub mod toast {
        pub const CONTAINER: &str = "fixed bottom-5 right-5 z-50 flex flex-col gap-2";
        pub const ITEM: &str = "flex items-center w-full max-w-xs p-4 text-gray-500 bg-white rounded-lg shadow dark:text-gray-400 dark:bg-gray-800 border border-gray-200 dark:border-gray-700";
    }

    pub mod accordion {
        pub const DEFAULT: &str = "bg-white dark:bg-gray-900 rounded-lg";
        pub const BORDERED: &str =
            "bg-white dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700";
        pub const GHOST: &str = "bg-transparent dark:bg-transparent rounded-none";
    }

    pub mod table {
        pub const DEFAULT: &str = "min-w-full divide-y divide-gray-200 dark:divide-gray-700";
        pub const STRIPED: &str = "min-w-full divide-y divide-gray-200 dark:divide-gray-700";
        pub const BORDERED: &str = "min-w-full border border-gray-200 dark:border-gray-700";
    }

    pub mod progress {
        pub const SIZE_SM: &str = "w-full";
        pub const SIZE_MD: &str = "w-full";
        pub const SIZE_LG: &str = "w-full";
        pub const DEFAULT: &str = "";
        pub const SUCCESS: &str = "";
        pub const WARNING: &str = "";
        pub const ERROR: &str = "";
    }
}
