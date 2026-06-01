# Annex L: Annex L

## Annex L
(normative)
vpi_compatibility.h
L.1 General
This include file contains special macro definitions required to support VPI compatibility mode
functionality (see 36.12, especially 36.12.2.1). It is automatically included by vpi_user.h (see AnnexK), and
therefore should not be included directly from user application code.
L.2 Source code
/*******************************************************************************
* vpi_compatibility.h
*
* IEEE Std 1800-2023 SystemVerilog Verification Procedural Interface (VPI)
*
* NOTE: THIS FILE IS INCLUDED BY vpi_user.h. DO NOT INCLUDE THIS FILE FROM
* USER APPLICATION CODE.
*
* This file contains the macro definitions used by the SystemVerilog PLI
* to implement backwards compatibility mode functionality.
*
******************************************************************************/
#ifdef VPI_COMPATIBILITY_H
#error "The vpi_compatibility.h file can only be included by vpi_user.h
directly."
#endif
#define VPI_COMPATIBILITY_H
/* Compatibility-mode variants of functions */
#if VPI_COMPATIBILITY_VERSION_1800v2023
#define VPI_COMPATIBILITY_VERSION_1800v2012
#endif
#if VPI_COMPATIBILITY_VERSION_1800v2017
#define VPI_COMPATIBILITY_VERSION_1800v2012
#endif
#if VPI_COMPATIBILITY_VERSION_1364v1995
#if VPI_COMPATIBILITY_VERSION_1364v2001 || VPI_COMPATIBILITY_VERSION_1364v2005
|| VPI_COMPATIBILITY_VERSION_1800v2005 || VPI_COMPATIBILITY_VERSION_1800v2009
|| VPI_COMPATIBILITY_VERSION_1800v2012
#error "Only one VPI_COMPATIBILITY_VERSION symbol definition is allowed."
#endif
#define vpi_compare_objects vpi_compare_objects_1364v1995
#define vpi_control vpi_control_1364v1995
#define vpi_get vpi_get_1364v1995
#define vpi_get_str vpi_get_str_1364v1995
#define vpi_get_value vpi_get_value_1364v1995
#define vpi_handle vpi_handle_1364v1995
#define vpi_handle_by_index vpi_handle_by_index_1364v1995
#define vpi_handle_by_multi_index vpi_handle_by_multi_index_1364v1995
#define vpi_handle_by_name vpi_handle_by_name_1364v1995
#define vpi_handle_multi vpi_handle_multi_1364v1995
#define vpi_iterate vpi_iterate_1364v1995
#define vpi_put_value vpi_put_value_1364v1995
#define vpi_register_cb vpi_register_cb_1364v1995

<!-- Page 1325 -->

IEEE Std #define vpi_scan vpi_scan_1364v1995
#elif VPI_COMPATIBILITY_VERSION_1364v2001
#if VPI_COMPATIBILITY_VERSION_1364v1995 || VPI_COMPATIBILITY_VERSION_1364v2005
|| VPI_COMPATIBILITY_VERSION_1800v2005 || VPI_COMPATIBILITY_VERSION_1800v2009
|| VPI_COMPATIBILITY_VERSION_1800v2012
#error "Only one VPI_COMPATIBILITY_VERSION symbol definition is allowed."
#endif
#define vpi_compare_objects vpi_compare_objects_1364v2001
#define vpi_control vpi_control_1364v2001
#define vpi_get vpi_get_1364v2001
#define vpi_get_str vpi_get_str_1364v2001
#define vpi_get_value vpi_get_value_1364v2001
#define vpi_handle vpi_handle_1364v2001
#define vpi_handle_by_index vpi_handle_by_index_1364v2001
#define vpi_handle_by_multi_index vpi_handle_by_multi_index_1364v2001
#define vpi_handle_by_name vpi_handle_by_name_1364v2001
#define vpi_handle_multi vpi_handle_multi_1364v2001
#define vpi_iterate vpi_iterate_1364v2001
#define vpi_put_value vpi_put_value_1364v2001
#define vpi_register_cb vpi_register_cb_1364v2001
#define vpi_scan vpi_scan_1364v2001
#elif VPI_COMPATIBILITY_VERSION_1364v2005
#if VPI_COMPATIBILITY_VERSION_1364v1995 || VPI_COMPATIBILITY_VERSION_1364v2001
|| VPI_COMPATIBILITY_VERSION_1800v2005 || VPI_COMPATIBILITY_VERSION_1800v2009
|| VPI_COMPATIBILITY_VERSION_1800v2012
#error "Only one VPI_COMPATIBILITY_VERSION symbol definition is allowed."
#endif
#define vpi_compare_objects vpi_compare_objects_1364v2005
#define vpi_control vpi_control_1364v2005
#define vpi_get vpi_get_1364v2005
#define vpi_get_str vpi_get_str_1364v2005
#define vpi_get_value vpi_get_value_1364v2005
#define vpi_handle vpi_handle_1364v2005
#define vpi_handle_by_index vpi_handle_by_index_1364v2005
#define vpi_handle_by_multi_index vpi_handle_by_multi_index_1364v2005
#define vpi_handle_by_name vpi_handle_by_name_1364v2005
#define vpi_handle_multi vpi_handle_multi_1364v2005
#define vpi_iterate vpi_iterate_1364v2005
#define vpi_put_value vpi_put_value_1364v2005
#define vpi_register_cb vpi_register_cb_1364v2005
#define vpi_scan vpi_scan_1364v2005
#elif VPI_COMPATIBILITY_VERSION_1800v2005
#if VPI_COMPATIBILITY_VERSION_1364v1995 || VPI_COMPATIBILITY_VERSION_1364v2001
|| VPI_COMPATIBILITY_VERSION_1364v2005 || VPI_COMPATIBILITY_VERSION_1800v2009
|| VPI_COMPATIBILITY_VERSION_1800v2012
#error "Only one VPI_COMPATIBILITY_VERSION symbol definition is allowed."
#endif
#define vpi_compare_objects vpi_compare_objects_1800v2005
#define vpi_control vpi_control_1800v2005
#define vpi_get vpi_get_1800v2005
#define vpi_get_str vpi_get_str_1800v2005
#define vpi_get_value vpi_get_value_1800v2005
#define vpi_handle vpi_handle_1800v2005
#define vpi_handle_by_index vpi_handle_by_index_1800v2005
#define vpi_handle_by_multi_index vpi_handle_by_multi_index_1800v2005
#define vpi_handle_by_name vpi_handle_by_name_1800v2005
#define vpi_handle_multi vpi_handle_multi_1800v2005
#define vpi_iterate vpi_iterate_1800v2005
#define vpi_put_value vpi_put_value_1800v2005
#define vpi_register_cb vpi_register_cb_1800v2005
#define vpi_scan vpi_scan_1800v2005
#elif VPI_COMPATIBILITY_VERSION_1800v2009
#if VPI_COMPATIBILITY_VERSION_1364v1995 || VPI_COMPATIBILITY_VERSION_1364v2001
|| VPI_COMPATIBILITY_VERSION_1364v2005 || VPI_COMPATIBILITY_VERSION_1800v2005

<!-- Page 1326 -->

IEEE Std || VPI_COMPATIBILITY_VERSION_1800v2012
#error "Only one VPI_COMPATIBILITY_VERSION symbol definition is allowed."
#endif
#define vpi_compare_objects vpi_compare_objects_1800v2009
#define vpi_control vpi_control_1800v2009
#define vpi_get vpi_get_1800v2009
#define vpi_get_str vpi_get_str_1800v2009
#define vpi_get_value vpi_get_value_1800v2009
#define vpi_handle vpi_handle_1800v2009
#define vpi_handle_by_index vpi_handle_by_index_1800v2009
#define vpi_handle_by_multi_index vpi_handle_by_multi_index_1800v2009
#define vpi_handle_by_name vpi_handle_by_name_1800v2009
#define vpi_handle_multi vpi_handle_multi_1800v2009
#define vpi_iterate vpi_iterate_1800v2009
#define vpi_put_value vpi_put_value_1800v2009
#define vpi_register_cb vpi_register_cb_1800v2009
#define vpi_scan vpi_scan_1800v2009
#elif VPI_COMPATIBILITY_VERSION_1800v2012
#if VPI_COMPATIBILITY_VERSION_1364v1995 || VPI_COMPATIBILITY_VERSION_1364v2001
|| VPI_COMPATIBILITY_VERSION_1364v2005 || VPI_COMPATIBILITY_VERSION_1800v2005
|| VPI_COMPATIBILITY_VERSION_1800v2009
#error "Only one VPI_COMPATIBILITY_VERSION symbol definition is allowed."
#endif
#define vpi_compare_objects vpi_compare_objects_1800v2012
#define vpi_control vpi_control_1800v2012
#define vpi_get vpi_get_1800v2012
#define vpi_get_str vpi_get_str_1800v2012
#define vpi_get_value vpi_get_value_1800v2012
#define vpi_handle vpi_handle_1800v2012
#define vpi_handle_by_index vpi_handle_by_index_1800v2012
#define vpi_handle_by_multi_index vpi_handle_by_multi_index_1800v2012
#define vpi_handle_by_name vpi_handle_by_name_1800v2012
#define vpi_handle_multi vpi_handle_multi_1800v2012
#define vpi_iterate vpi_iterate_1800v2012
#define vpi_put_value vpi_put_value_1800v2012
#define vpi_register_cb vpi_register_cb_1800v2012
#define vpi_scan vpi_scan_1800v2012
#endif

<!-- Page 1327 -->

IEEE Std 1800-2023