#import "WalletPlugin.h"

#import <Foundation/Foundation.h>

#include <memory.h>
#include <stdint.h>

#include "LibWallet.h"

@implementation WalletPlugin

- (void)WALLET_RESTORE:(CDVInvokedUrlCommand*)command
{
    NSString* mnemonics = [command.arguments objectAtIndex:0];

    [self.commandDelegate runInBackground:^{
        CDVPluginResult* pluginResult = nil;

        WalletPtr wallet_ptr;
        ErrorPtr result =
            iohk_jormungandr_wallet_recover([mnemonics UTF8String], nil, 0, &wallet_ptr);

        if (result != nil) {
            char* error_desc_raw = iohk_jormungandr_wallet_error_to_string(result);      
            NSString* error_desc = [NSString stringWithCString:error_desc_raw
                                                      encoding:NSUTF8StringEncoding];    

            pluginResult = [CDVPluginResult resultWithStatus:CDVCommandStatus_ERROR
                                             messageAsString:error_desc];

            free(error_desc_raw);
            iohk_jormungandr_wallet_delete_error(result);
        } else {
            NSString* returnValue = [NSString stringWithFormat:@"%ld", (uintptr_t)wallet_ptr];
            pluginResult = [CDVPluginResult resultWithStatus:CDVCommandStatus_OK
                                             messageAsString:returnValue];
        }

        [self.commandDelegate sendPluginResult:pluginResult callbackId:command.callbackId];
    }];
}

- (void)WALLET_RETRIEVE_FUNDS:(CDVInvokedUrlCommand*)command
{
    NSString* wallet_ptr_raw = [command.arguments objectAtIndex:0];
    NSString* raw_block0 = [command.arguments objectAtIndex:1];

    WalletPtr wallet_ptr = (WalletPtr)[wallet_ptr_raw longLongValue];
    NSData* block0 = [[NSData alloc] initWithBase64EncodedString:raw_block0 options:0];

    [self.commandDelegate runInBackground:^{
        CDVPluginResult* pluginResult = nil;

        SettingsPtr settings_ptr = nil;
        ErrorPtr result = iohk_jormungandr_wallet_retrieve_funds(wallet_ptr,
            block0.bytes,
            block0.length,
            &settings_ptr);

        if (result != nil) {
            char* error_desc_raw = iohk_jormungandr_wallet_error_to_string(result);
            NSString* error_desc = [NSString stringWithCString:error_desc_raw
                                                      encoding:NSUTF8StringEncoding];

            pluginResult = [CDVPluginResult resultWithStatus:CDVCommandStatus_ERROR
                                             messageAsString:error_desc];

            free(error_desc_raw);
            iohk_jormungandr_wallet_delete_error(result);
        } else {
            NSString* returnValue = [NSString stringWithFormat:@"%ld", (uintptr_t)settings_ptr];
            pluginResult = [CDVPluginResult resultWithStatus:CDVCommandStatus_OK
                                             messageAsString:returnValue];
        }

        [self.commandDelegate sendPluginResult:pluginResult callbackId:command.callbackId];
    }];
}

- (void)WALLET_TOTAL_FUNDS:(CDVInvokedUrlCommand*)command
{
    CDVPluginResult* pluginResult = nil;
    NSString* wallet_ptr_raw = [command.arguments objectAtIndex:0];

    WalletPtr wallet_ptr = (WalletPtr)[wallet_ptr_raw longLongValue];
    uint64_t value;
    ErrorPtr result = iohk_jormungandr_wallet_total_value(wallet_ptr, &value);

    if (result != nil) {
        char* error_desc_raw = iohk_jormungandr_wallet_error_to_string(result);
        NSString* error_desc = [NSString stringWithCString:error_desc_raw
                                                  encoding:NSUTF8StringEncoding];

        pluginResult = [CDVPluginResult resultWithStatus:CDVCommandStatus_ERROR
                                         messageAsString:error_desc];

        free(error_desc_raw);
        iohk_jormungandr_wallet_delete_error(result);
    } else {
        NSString* returnValue = [NSString stringWithFormat:@"%lld", value];
        pluginResult = [CDVPluginResult resultWithStatus:CDVCommandStatus_OK
                                         messageAsString:returnValue];
    }

    [self.commandDelegate sendPluginResult:pluginResult callbackId:command.callbackId];
}

- (void)WALLET_ID:(CDVInvokedUrlCommand*)command
{
    CDVPluginResult* pluginResult = nil;
    NSString* wallet_ptr_raw = [command.arguments objectAtIndex:0];

    WalletPtr wallet_ptr = (WalletPtr)[wallet_ptr_raw longLongValue];
    uint8_t id_ptr[32];
    ErrorPtr result = iohk_jormungandr_wallet_id(wallet_ptr, id_ptr);

    if (result != nil) {
        char* error_desc_raw = iohk_jormungandr_wallet_error_to_string(result);
        NSString* error_desc = [NSString stringWithCString:error_desc_raw
                                                  encoding:NSUTF8StringEncoding];

        pluginResult = [CDVPluginResult resultWithStatus:CDVCommandStatus_ERROR
                                         messageAsString:error_desc];

        free(error_desc_raw);
        iohk_jormungandr_wallet_delete_error(result);
    } else {
        NSData* returnValue = [NSData dataWithBytes:id_ptr length:32];
        pluginResult = [CDVPluginResult resultWithStatus:CDVCommandStatus_OK
                                    messageAsArrayBuffer:returnValue];
    }

    [self.commandDelegate sendPluginResult:pluginResult callbackId:command.callbackId];
}

- (void)WALLET_CONVERT:(CDVInvokedUrlCommand*)command
{
    CDVPluginResult* pluginResult = nil;
    NSString* wallet_ptr_raw = [command.arguments objectAtIndex:0];
    NSString* settings_ptr_raw = [command.arguments objectAtIndex:1];

    WalletPtr wallet_ptr = (WalletPtr)[wallet_ptr_raw longLongValue];
    SettingsPtr settings_ptr = (SettingsPtr)[settings_ptr_raw longLongValue];

    ConversionPtr conversion_ptr = nil;
    ErrorPtr result = iohk_jormungandr_wallet_convert(wallet_ptr, settings_ptr, &conversion_ptr);

    if (result != nil) {
        char* error_desc_raw = iohk_jormungandr_wallet_error_to_string(result);
        NSString* error_desc = [NSString stringWithCString:error_desc_raw
                                                  encoding:NSUTF8StringEncoding];

        pluginResult = [CDVPluginResult resultWithStatus:CDVCommandStatus_ERROR
                                         messageAsString:error_desc];

        free(error_desc_raw);
        iohk_jormungandr_wallet_delete_error(result);
    } else {
        NSString* returnValue = [NSString stringWithFormat:@"%ld", (uintptr_t)conversion_ptr];
        pluginResult = [CDVPluginResult resultWithStatus:CDVCommandStatus_OK
                                         messageAsString:returnValue];
    }

    [self.commandDelegate sendPluginResult:pluginResult callbackId:command.callbackId];
}

- (void)CONVERSION_TRANSACTIONS_SIZE:(CDVInvokedUrlCommand*)command
{
    NSString* conversion_ptr_raw = [command.arguments objectAtIndex:0];
    ConversionPtr conversion_ptr = (ConversionPtr)[conversion_ptr_raw longLongValue];
    uintptr_t value = iohk_jormungandr_wallet_convert_transactions_size(conversion_ptr);
    NSString* returnValue = [NSString stringWithFormat:@"%ld", (uintptr_t)value];
    CDVPluginResult* pluginResult = [CDVPluginResult resultWithStatus:CDVCommandStatus_OK
                                                      messageAsString:returnValue];
    [self.commandDelegate sendPluginResult:pluginResult callbackId:command.callbackId];
}

- (void)CONVERSION_TRANSACTIONS_GET:(CDVInvokedUrlCommand*)command
{
    CDVPluginResult* pluginResult = nil;
    NSString* conversion_ptr_raw = [command.arguments objectAtIndex:0];
    NSString* index_raw = [command.arguments objectAtIndex:0];

    ConversionPtr conversion_ptr = (ConversionPtr)[conversion_ptr_raw longLongValue];
    uintptr_t index = (uintptr_t)[index_raw longLongValue];

    uint8_t* transaction_out_ptr = nil;
    uintptr_t transaction_size;

    ErrorPtr result = iohk_jormungandr_wallet_convert_transactions_get(conversion_ptr,
        index,
        &transaction_out_ptr,
        &transaction_size);

    if (result != nil) {
        char* error_desc_raw = iohk_jormungandr_wallet_error_to_string(result);
        NSString* error_desc = [NSString stringWithCString:error_desc_raw
                                                  encoding:NSUTF8StringEncoding];

        pluginResult = [CDVPluginResult resultWithStatus:CDVCommandStatus_ERROR
                                         messageAsString:error_desc];

        free(error_desc_raw);
        iohk_jormungandr_wallet_delete_error(result);
    } else {
        NSData* returnValue = [NSData dataWithBytes:transaction_out_ptr length:transaction_size];
        pluginResult = [CDVPluginResult resultWithStatus:CDVCommandStatus_OK
                                    messageAsArrayBuffer:returnValue];
    }
}

- (void)WALLET_DELETE:(CDVInvokedUrlCommand*)command
{
    NSString* wallet_ptr_raw = [command.arguments objectAtIndex:0];
    WalletPtr wallet_ptr = (WalletPtr)[wallet_ptr_raw longLongValue];
    iohk_jormungandr_wallet_delete_wallet(wallet_ptr);
    CDVPluginResult* pluginResult = [CDVPluginResult resultWithStatus:CDVCommandStatus_OK];
    [self.commandDelegate sendPluginResult:pluginResult callbackId:command.callbackId];
}

- (void)SETTINGS_DELETE:(CDVInvokedUrlCommand*)command
{
    NSString* settings_ptr_raw = [command.arguments objectAtIndex:0];
    SettingsPtr settings_ptr = (SettingsPtr)[settings_ptr_raw longLongValue];
    iohk_jormungandr_wallet_delete_settings(settings_ptr);
    CDVPluginResult* pluginResult = [CDVPluginResult resultWithStatus:CDVCommandStatus_OK];
    [self.commandDelegate sendPluginResult:pluginResult callbackId:command.callbackId];
}

- (void)CONVERSION_DELETE:(CDVInvokedUrlCommand*)command
{
    NSString* conversion_ptr_raw = [command.arguments objectAtIndex:0];
    ConversionPtr conversion_ptr = (ConversionPtr)[conversion_ptr_raw longLongValue];
    iohk_jormungandr_wallet_delete_conversion(conversion_ptr);
    CDVPluginResult* pluginResult = [CDVPluginResult resultWithStatus:CDVCommandStatus_OK];
    [self.commandDelegate sendPluginResult:pluginResult callbackId:command.callbackId];
}

@end

