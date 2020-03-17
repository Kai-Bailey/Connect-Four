"use strict";

if( typeof Rust === "undefined" ) {
    var Rust = {};
}

(function( root, factory ) {
    if( typeof define === "function" && define.amd ) {
        define( [], factory );
    } else if( typeof module === "object" && module.exports ) {
        module.exports = factory();
    } else {
        Rust.connect_four_frontend = factory();
    }
}( this, function() {
    return (function( module_factory ) {
        var instance = module_factory();

        if( typeof process === "object" && typeof process.versions === "object" && typeof process.versions.node === "string" ) {
            var fs = require( "fs" );
            var path = require( "path" );
            var wasm_path = path.join( __dirname, "connect-four-frontend.wasm" );
            var buffer = fs.readFileSync( wasm_path );
            var mod = new WebAssembly.Module( buffer );
            var wasm_instance = new WebAssembly.Instance( mod, instance.imports );
            return instance.initialize( wasm_instance );
        } else {
            var file = fetch( "connect-four-frontend.wasm", {credentials: "same-origin"} );

            var wasm_instance = ( typeof WebAssembly.instantiateStreaming === "function"
                ? WebAssembly.instantiateStreaming( file, instance.imports )
                    .then( function( result ) { return result.instance; } )

                : file
                    .then( function( response ) { return response.arrayBuffer(); } )
                    .then( function( bytes ) { return WebAssembly.compile( bytes ); } )
                    .then( function( mod ) { return WebAssembly.instantiate( mod, instance.imports ) } ) );

            return wasm_instance
                .then( function( wasm_instance ) {
                    var exports = instance.initialize( wasm_instance );
                    console.log( "Finished loading Rust wasm module 'connect_four_frontend'" );
                    return exports;
                })
                .catch( function( error ) {
                    console.log( "Error loading Rust wasm module 'connect_four_frontend':", error );
                    throw error;
                });
        }
    }( function() {
    var Module = {};

    Module.STDWEB_PRIVATE = {};

// This is based on code from Emscripten's preamble.js.
Module.STDWEB_PRIVATE.to_utf8 = function to_utf8( str, addr ) {
    var HEAPU8 = Module.HEAPU8;
    for( var i = 0; i < str.length; ++i ) {
        // Gotcha: charCodeAt returns a 16-bit word that is a UTF-16 encoded code unit, not a Unicode code point of the character! So decode UTF16->UTF32->UTF8.
        // See http://unicode.org/faq/utf_bom.html#utf16-3
        // For UTF8 byte structure, see http://en.wikipedia.org/wiki/UTF-8#Description and https://www.ietf.org/rfc/rfc2279.txt and https://tools.ietf.org/html/rfc3629
        var u = str.charCodeAt( i ); // possibly a lead surrogate
        if( u >= 0xD800 && u <= 0xDFFF ) {
            u = 0x10000 + ((u & 0x3FF) << 10) | (str.charCodeAt( ++i ) & 0x3FF);
        }

        if( u <= 0x7F ) {
            HEAPU8[ addr++ ] = u;
        } else if( u <= 0x7FF ) {
            HEAPU8[ addr++ ] = 0xC0 | (u >> 6);
            HEAPU8[ addr++ ] = 0x80 | (u & 63);
        } else if( u <= 0xFFFF ) {
            HEAPU8[ addr++ ] = 0xE0 | (u >> 12);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 6) & 63);
            HEAPU8[ addr++ ] = 0x80 | (u & 63);
        } else if( u <= 0x1FFFFF ) {
            HEAPU8[ addr++ ] = 0xF0 | (u >> 18);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 12) & 63);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 6) & 63);
            HEAPU8[ addr++ ] = 0x80 | (u & 63);
        } else if( u <= 0x3FFFFFF ) {
            HEAPU8[ addr++ ] = 0xF8 | (u >> 24);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 18) & 63);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 12) & 63);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 6) & 63);
            HEAPU8[ addr++ ] = 0x80 | (u & 63);
        } else {
            HEAPU8[ addr++ ] = 0xFC | (u >> 30);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 24) & 63);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 18) & 63);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 12) & 63);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 6) & 63);
            HEAPU8[ addr++ ] = 0x80 | (u & 63);
        }
    }
};

Module.STDWEB_PRIVATE.noop = function() {};
Module.STDWEB_PRIVATE.to_js = function to_js( address ) {
    var kind = Module.HEAPU8[ address + 12 ];
    if( kind === 0 ) {
        return undefined;
    } else if( kind === 1 ) {
        return null;
    } else if( kind === 2 ) {
        return Module.HEAP32[ address / 4 ];
    } else if( kind === 3 ) {
        return Module.HEAPF64[ address / 8 ];
    } else if( kind === 4 ) {
        var pointer = Module.HEAPU32[ address / 4 ];
        var length = Module.HEAPU32[ (address + 4) / 4 ];
        return Module.STDWEB_PRIVATE.to_js_string( pointer, length );
    } else if( kind === 5 ) {
        return false;
    } else if( kind === 6 ) {
        return true;
    } else if( kind === 7 ) {
        var pointer = Module.STDWEB_PRIVATE.arena + Module.HEAPU32[ address / 4 ];
        var length = Module.HEAPU32[ (address + 4) / 4 ];
        var output = [];
        for( var i = 0; i < length; ++i ) {
            output.push( Module.STDWEB_PRIVATE.to_js( pointer + i * 16 ) );
        }
        return output;
    } else if( kind === 8 ) {
        var arena = Module.STDWEB_PRIVATE.arena;
        var value_array_pointer = arena + Module.HEAPU32[ address / 4 ];
        var length = Module.HEAPU32[ (address + 4) / 4 ];
        var key_array_pointer = arena + Module.HEAPU32[ (address + 8) / 4 ];
        var output = {};
        for( var i = 0; i < length; ++i ) {
            var key_pointer = Module.HEAPU32[ (key_array_pointer + i * 8) / 4 ];
            var key_length = Module.HEAPU32[ (key_array_pointer + 4 + i * 8) / 4 ];
            var key = Module.STDWEB_PRIVATE.to_js_string( key_pointer, key_length );
            var value = Module.STDWEB_PRIVATE.to_js( value_array_pointer + i * 16 );
            output[ key ] = value;
        }
        return output;
    } else if( kind === 9 ) {
        return Module.STDWEB_PRIVATE.acquire_js_reference( Module.HEAP32[ address / 4 ] );
    } else if( kind === 10 || kind === 12 || kind === 13 ) {
        var adapter_pointer = Module.HEAPU32[ address / 4 ];
        var pointer = Module.HEAPU32[ (address + 4) / 4 ];
        var deallocator_pointer = Module.HEAPU32[ (address + 8) / 4 ];
        var num_ongoing_calls = 0;
        var drop_queued = false;
        var output = function() {
            if( pointer === 0 || drop_queued === true ) {
                if (kind === 10) {
                    throw new ReferenceError( "Already dropped Rust function called!" );
                } else if (kind === 12) {
                    throw new ReferenceError( "Already dropped FnMut function called!" );
                } else {
                    throw new ReferenceError( "Already called or dropped FnOnce function called!" );
                }
            }

            var function_pointer = pointer;
            if (kind === 13) {
                output.drop = Module.STDWEB_PRIVATE.noop;
                pointer = 0;
            }

            if (num_ongoing_calls !== 0) {
                if (kind === 12 || kind === 13) {
                    throw new ReferenceError( "FnMut function called multiple times concurrently!" );
                }
            }

            var args = Module.STDWEB_PRIVATE.alloc( 16 );
            Module.STDWEB_PRIVATE.serialize_array( args, arguments );

            try {
                num_ongoing_calls += 1;
                Module.STDWEB_PRIVATE.dyncall( "vii", adapter_pointer, [function_pointer, args] );
                var result = Module.STDWEB_PRIVATE.tmp;
                Module.STDWEB_PRIVATE.tmp = null;
            } finally {
                num_ongoing_calls -= 1;
            }

            if( drop_queued === true && num_ongoing_calls === 0 ) {
                output.drop();
            }

            return result;
        };

        output.drop = function() {
            if (num_ongoing_calls !== 0) {
                drop_queued = true;
                return;
            }

            output.drop = Module.STDWEB_PRIVATE.noop;
            var function_pointer = pointer;
            pointer = 0;

            if (function_pointer != 0) {
                Module.STDWEB_PRIVATE.dyncall( "vi", deallocator_pointer, [function_pointer] );
            }
        };

        return output;
    } else if( kind === 14 ) {
        var pointer = Module.HEAPU32[ address / 4 ];
        var length = Module.HEAPU32[ (address + 4) / 4 ];
        var array_kind = Module.HEAPU32[ (address + 8) / 4 ];
        var pointer_end = pointer + length;

        switch( array_kind ) {
            case 0:
                return Module.HEAPU8.subarray( pointer, pointer_end );
            case 1:
                return Module.HEAP8.subarray( pointer, pointer_end );
            case 2:
                return Module.HEAPU16.subarray( pointer, pointer_end );
            case 3:
                return Module.HEAP16.subarray( pointer, pointer_end );
            case 4:
                return Module.HEAPU32.subarray( pointer, pointer_end );
            case 5:
                return Module.HEAP32.subarray( pointer, pointer_end );
            case 6:
                return Module.HEAPF32.subarray( pointer, pointer_end );
            case 7:
                return Module.HEAPF64.subarray( pointer, pointer_end );
        }
    } else if( kind === 15 ) {
        return Module.STDWEB_PRIVATE.get_raw_value( Module.HEAPU32[ address / 4 ] );
    }
};

Module.STDWEB_PRIVATE.serialize_object = function serialize_object( address, value ) {
    var keys = Object.keys( value );
    var length = keys.length;
    var key_array_pointer = Module.STDWEB_PRIVATE.alloc( length * 8 );
    var value_array_pointer = Module.STDWEB_PRIVATE.alloc( length * 16 );
    Module.HEAPU8[ address + 12 ] = 8;
    Module.HEAPU32[ address / 4 ] = value_array_pointer;
    Module.HEAPU32[ (address + 4) / 4 ] = length;
    Module.HEAPU32[ (address + 8) / 4 ] = key_array_pointer;
    for( var i = 0; i < length; ++i ) {
        var key = keys[ i ];
        var key_address = key_array_pointer + i * 8;
        Module.STDWEB_PRIVATE.to_utf8_string( key_address, key );

        Module.STDWEB_PRIVATE.from_js( value_array_pointer + i * 16, value[ key ] );
    }
};

Module.STDWEB_PRIVATE.serialize_array = function serialize_array( address, value ) {
    var length = value.length;
    var pointer = Module.STDWEB_PRIVATE.alloc( length * 16 );
    Module.HEAPU8[ address + 12 ] = 7;
    Module.HEAPU32[ address / 4 ] = pointer;
    Module.HEAPU32[ (address + 4) / 4 ] = length;
    for( var i = 0; i < length; ++i ) {
        Module.STDWEB_PRIVATE.from_js( pointer + i * 16, value[ i ] );
    }
};

// New browsers and recent Node
var cachedEncoder = ( typeof TextEncoder === "function"
    ? new TextEncoder( "utf-8" )
    // Old Node (before v11)
    : ( typeof util === "object" && util && typeof util.TextEncoder === "function"
        ? new util.TextEncoder( "utf-8" )
        // Old browsers
        : null ) );

if ( cachedEncoder != null ) {
    Module.STDWEB_PRIVATE.to_utf8_string = function to_utf8_string( address, value ) {
        var buffer = cachedEncoder.encode( value );
        var length = buffer.length;
        var pointer = 0;

        if ( length > 0 ) {
            pointer = Module.STDWEB_PRIVATE.alloc( length );
            Module.HEAPU8.set( buffer, pointer );
        }

        Module.HEAPU32[ address / 4 ] = pointer;
        Module.HEAPU32[ (address + 4) / 4 ] = length;
    };

} else {
    Module.STDWEB_PRIVATE.to_utf8_string = function to_utf8_string( address, value ) {
        var length = Module.STDWEB_PRIVATE.utf8_len( value );
        var pointer = 0;

        if ( length > 0 ) {
            pointer = Module.STDWEB_PRIVATE.alloc( length );
            Module.STDWEB_PRIVATE.to_utf8( value, pointer );
        }

        Module.HEAPU32[ address / 4 ] = pointer;
        Module.HEAPU32[ (address + 4) / 4 ] = length;
    };
}

Module.STDWEB_PRIVATE.from_js = function from_js( address, value ) {
    var kind = Object.prototype.toString.call( value );
    if( kind === "[object String]" ) {
        Module.HEAPU8[ address + 12 ] = 4;
        Module.STDWEB_PRIVATE.to_utf8_string( address, value );
    } else if( kind === "[object Number]" ) {
        if( value === (value|0) ) {
            Module.HEAPU8[ address + 12 ] = 2;
            Module.HEAP32[ address / 4 ] = value;
        } else {
            Module.HEAPU8[ address + 12 ] = 3;
            Module.HEAPF64[ address / 8 ] = value;
        }
    } else if( value === null ) {
        Module.HEAPU8[ address + 12 ] = 1;
    } else if( value === undefined ) {
        Module.HEAPU8[ address + 12 ] = 0;
    } else if( value === false ) {
        Module.HEAPU8[ address + 12 ] = 5;
    } else if( value === true ) {
        Module.HEAPU8[ address + 12 ] = 6;
    } else if( kind === "[object Symbol]" ) {
        var id = Module.STDWEB_PRIVATE.register_raw_value( value );
        Module.HEAPU8[ address + 12 ] = 15;
        Module.HEAP32[ address / 4 ] = id;
    } else {
        var refid = Module.STDWEB_PRIVATE.acquire_rust_reference( value );
        Module.HEAPU8[ address + 12 ] = 9;
        Module.HEAP32[ address / 4 ] = refid;
    }
};

// New browsers and recent Node
var cachedDecoder = ( typeof TextDecoder === "function"
    ? new TextDecoder( "utf-8" )
    // Old Node (before v11)
    : ( typeof util === "object" && util && typeof util.TextDecoder === "function"
        ? new util.TextDecoder( "utf-8" )
        // Old browsers
        : null ) );

if ( cachedDecoder != null ) {
    Module.STDWEB_PRIVATE.to_js_string = function to_js_string( index, length ) {
        return cachedDecoder.decode( Module.HEAPU8.subarray( index, index + length ) );
    };

} else {
    // This is ported from Rust's stdlib; it's faster than
    // the string conversion from Emscripten.
    Module.STDWEB_PRIVATE.to_js_string = function to_js_string( index, length ) {
        var HEAPU8 = Module.HEAPU8;
        index = index|0;
        length = length|0;
        var end = (index|0) + (length|0);
        var output = "";
        while( index < end ) {
            var x = HEAPU8[ index++ ];
            if( x < 128 ) {
                output += String.fromCharCode( x );
                continue;
            }
            var init = (x & (0x7F >> 2));
            var y = 0;
            if( index < end ) {
                y = HEAPU8[ index++ ];
            }
            var ch = (init << 6) | (y & 63);
            if( x >= 0xE0 ) {
                var z = 0;
                if( index < end ) {
                    z = HEAPU8[ index++ ];
                }
                var y_z = ((y & 63) << 6) | (z & 63);
                ch = init << 12 | y_z;
                if( x >= 0xF0 ) {
                    var w = 0;
                    if( index < end ) {
                        w = HEAPU8[ index++ ];
                    }
                    ch = (init & 7) << 18 | ((y_z << 6) | (w & 63));

                    output += String.fromCharCode( 0xD7C0 + (ch >> 10) );
                    ch = 0xDC00 + (ch & 0x3FF);
                }
            }
            output += String.fromCharCode( ch );
            continue;
        }
        return output;
    };
}

Module.STDWEB_PRIVATE.id_to_ref_map = {};
Module.STDWEB_PRIVATE.id_to_refcount_map = {};
Module.STDWEB_PRIVATE.ref_to_id_map = new WeakMap();
// Not all types can be stored in a WeakMap
Module.STDWEB_PRIVATE.ref_to_id_map_fallback = new Map();
Module.STDWEB_PRIVATE.last_refid = 1;

Module.STDWEB_PRIVATE.id_to_raw_value_map = {};
Module.STDWEB_PRIVATE.last_raw_value_id = 1;

Module.STDWEB_PRIVATE.acquire_rust_reference = function( reference ) {
    if( reference === undefined || reference === null ) {
        return 0;
    }

    var id_to_refcount_map = Module.STDWEB_PRIVATE.id_to_refcount_map;
    var id_to_ref_map = Module.STDWEB_PRIVATE.id_to_ref_map;
    var ref_to_id_map = Module.STDWEB_PRIVATE.ref_to_id_map;
    var ref_to_id_map_fallback = Module.STDWEB_PRIVATE.ref_to_id_map_fallback;

    var refid = ref_to_id_map.get( reference );
    if( refid === undefined ) {
        refid = ref_to_id_map_fallback.get( reference );
    }
    if( refid === undefined ) {
        refid = Module.STDWEB_PRIVATE.last_refid++;
        try {
            ref_to_id_map.set( reference, refid );
        } catch (e) {
            ref_to_id_map_fallback.set( reference, refid );
        }
    }

    if( refid in id_to_ref_map ) {
        id_to_refcount_map[ refid ]++;
    } else {
        id_to_ref_map[ refid ] = reference;
        id_to_refcount_map[ refid ] = 1;
    }

    return refid;
};

Module.STDWEB_PRIVATE.acquire_js_reference = function( refid ) {
    return Module.STDWEB_PRIVATE.id_to_ref_map[ refid ];
};

Module.STDWEB_PRIVATE.increment_refcount = function( refid ) {
    Module.STDWEB_PRIVATE.id_to_refcount_map[ refid ]++;
};

Module.STDWEB_PRIVATE.decrement_refcount = function( refid ) {
    var id_to_refcount_map = Module.STDWEB_PRIVATE.id_to_refcount_map;
    if( 0 == --id_to_refcount_map[ refid ] ) {
        var id_to_ref_map = Module.STDWEB_PRIVATE.id_to_ref_map;
        var ref_to_id_map_fallback = Module.STDWEB_PRIVATE.ref_to_id_map_fallback;
        var reference = id_to_ref_map[ refid ];
        delete id_to_ref_map[ refid ];
        delete id_to_refcount_map[ refid ];
        ref_to_id_map_fallback.delete(reference);
    }
};

Module.STDWEB_PRIVATE.register_raw_value = function( value ) {
    var id = Module.STDWEB_PRIVATE.last_raw_value_id++;
    Module.STDWEB_PRIVATE.id_to_raw_value_map[ id ] = value;
    return id;
};

Module.STDWEB_PRIVATE.unregister_raw_value = function( id ) {
    delete Module.STDWEB_PRIVATE.id_to_raw_value_map[ id ];
};

Module.STDWEB_PRIVATE.get_raw_value = function( id ) {
    return Module.STDWEB_PRIVATE.id_to_raw_value_map[ id ];
};

Module.STDWEB_PRIVATE.alloc = function alloc( size ) {
    return Module.web_malloc( size );
};

Module.STDWEB_PRIVATE.dyncall = function( signature, ptr, args ) {
    return Module.web_table.get( ptr ).apply( null, args );
};

// This is based on code from Emscripten's preamble.js.
Module.STDWEB_PRIVATE.utf8_len = function utf8_len( str ) {
    var len = 0;
    for( var i = 0; i < str.length; ++i ) {
        // Gotcha: charCodeAt returns a 16-bit word that is a UTF-16 encoded code unit, not a Unicode code point of the character! So decode UTF16->UTF32->UTF8.
        // See http://unicode.org/faq/utf_bom.html#utf16-3
        var u = str.charCodeAt( i ); // possibly a lead surrogate
        if( u >= 0xD800 && u <= 0xDFFF ) {
            u = 0x10000 + ((u & 0x3FF) << 10) | (str.charCodeAt( ++i ) & 0x3FF);
        }

        if( u <= 0x7F ) {
            ++len;
        } else if( u <= 0x7FF ) {
            len += 2;
        } else if( u <= 0xFFFF ) {
            len += 3;
        } else if( u <= 0x1FFFFF ) {
            len += 4;
        } else if( u <= 0x3FFFFFF ) {
            len += 5;
        } else {
            len += 6;
        }
    }
    return len;
};

Module.STDWEB_PRIVATE.prepare_any_arg = function( value ) {
    var arg = Module.STDWEB_PRIVATE.alloc( 16 );
    Module.STDWEB_PRIVATE.from_js( arg, value );
    return arg;
};

Module.STDWEB_PRIVATE.acquire_tmp = function( dummy ) {
    var value = Module.STDWEB_PRIVATE.tmp;
    Module.STDWEB_PRIVATE.tmp = null;
    return value;
};



    var HEAP8 = null;
    var HEAP16 = null;
    var HEAP32 = null;
    var HEAPU8 = null;
    var HEAPU16 = null;
    var HEAPU32 = null;
    var HEAPF32 = null;
    var HEAPF64 = null;

    Object.defineProperty( Module, 'exports', { value: {} } );

    function __web_on_grow() {
        var buffer = Module.instance.exports.memory.buffer;
        HEAP8 = new Int8Array( buffer );
        HEAP16 = new Int16Array( buffer );
        HEAP32 = new Int32Array( buffer );
        HEAPU8 = new Uint8Array( buffer );
        HEAPU16 = new Uint16Array( buffer );
        HEAPU32 = new Uint32Array( buffer );
        HEAPF32 = new Float32Array( buffer );
        HEAPF64 = new Float64Array( buffer );
        Module.HEAP8 = HEAP8;
        Module.HEAP16 = HEAP16;
        Module.HEAP32 = HEAP32;
        Module.HEAPU8 = HEAPU8;
        Module.HEAPU16 = HEAPU16;
        Module.HEAPU32 = HEAPU32;
        Module.HEAPF32 = HEAPF32;
        Module.HEAPF64 = HEAPF64;
    }

    return {
        imports: {
            env: {
                "__cargo_web_snippet_4ca6e16390bd7a77043d558deb813f2dc89828a0": function($0) {
                Module.STDWEB_PRIVATE.from_js($0, (function(){angular.module("Connect4App").controller("ScoreBoardCtrl",ScoreBoardCtrl);angular.module("Connect4App").factory("postService",function(S_resource){return S_resource("/games");});function ScoreBoardCtrl(postService,S_scope,S_rootScope){S_scope.games=postService.query();};})());
            },
            "__cargo_web_snippet_80d6d56760c65e49b7be8b6b01c1ea861b046bf0": function($0) {
                Module.STDWEB_PRIVATE.decrement_refcount( $0 );
            },
            "__cargo_web_snippet_a391aa6020e05c4c9607db5633198c9b40d34665": function($0) {
                Module.STDWEB_PRIVATE.from_js($0, (function(){angular.module("Connect4App").controller("humanController",humanController);app.factory("postService",function(S_resource){return S_resource("/games");});function humanController(postService,S_scope,S_rootScope){S_scope.games=[];S_scope.newGame={gameNumber:"100",gameType:"Connect-4",Player1Name:"",Player2Name:"",WinnerName:"Unfinished Game",GameDate:Date.now()};Array.prototype.clone=function(){var arr=[],i;for(i=0;i<this.length;i++){arr[i]=this[i].slice();}return arr;};S_scope.Game=function(){var target=document.getElementById("gameboard");var button=document.getElementById("startbutton");button.disabled=true;var target1=document.getElementById("textbox1");var target2=document.getElementById("textbox2");var that=this;this.map=[];this.paused=false;this.won=false;this.rejectClick=false;this.move=0;this.aiHistory=[];this.initOnceDone=false;this.initOnce=function(){if(this.initOnceDone){return false;}this.canvas=document.getElementsByTagName("canvas")[0];this.canvas.addEventListener("click",function(e){that.onclick(that.canvas,e);});this.context=this.canvas.getContext("2d");this.initOnceDone=true;};this.init=function(){this.map=[];this.paused=false;this.won=false;this.rejectClick=false;this.move=0;this.aiHistory=[];this.initOnce();var i,j;for(i=0;i<=6;i++){this.map[i]=[];for(j=0;j<=7;j++){this.map[i][j]=0;}}this.clear();this.drawMask();this.print();};this.playerMove=function(){if(this.move%2===0){return 1;}return-1;};this.print=function(){var i,j,msg;msg="\n";msg+="Move: "+this.move;msg+="\n";for(i=0;i<6;i++){for(j=0;j<7;j++){msg+=" "+this.map[i][j];}msg+="\n";}console.log(msg);};this.printState=function(state){var i,j,msg="\n";for(i=0;i<6;i++){for(j=0;j<7;j++){msg+=" "+state[i][j];}msg+="\n";}console.log(msg);};this.win=function(player){this.paused=true;this.won=true;this.rejectClick=false;var msg=null;if(player>0){msg=S_scope.newGame.Player1Name+" wins";S_scope.newGame.WinnerName=S_scope.newGame.Player1Name;}else if(player<0){msg=S_scope.newGame.Player2Name+" wins";S_scope.newGame.WinnerName=S_scope.newGame.Player2Name;}else{msg="It's a draw";S_scope.newGame.WinnerName="Draw";}msg+=" - Click on game board to reset";this.context.save();this.context.font="14pt sans-serif";this.context.fillStyle="#111";this.context.fillText(msg,150,20);postService.save(S_scope.newGame,function(){console.log("succesfully saved");});this.canvas=document.getElementsByTagName("canvas")[0];this.canvas.addEventListener("click",function(e){location.reload();});button.disabled=false;console.info(msg);};this.fillMap=function(state,column,value){var tempMap=state.clone();if(tempMap[0][column]!==0||column<0||column>6){return-1;}var done=false,row=0,i;for(i=0;i<5;i++){if(tempMap[i+1][column]!==0){done=true;row=i;break;}}if(! done){row=5;}tempMap[row][column]=value;return tempMap;};this.action=function(column,callback){if(this.paused||this.won){return 0;}if(this.map[0][column]!==0||column<0||column>6){return-1;}var done=false;var row=0,i;for(i=0;i<5;i++){if(this.map[i+1][column]!==0){done=true;row=i;break;}}if(! done){row=5;}this.animate(column,this.playerMove(this.move),row,0,function(){that.map[row][column]=that.playerMove(that.move);that.move++;that.draw();that.check();that.print();});this.paused=true;return 1;};this.check=function(){var i,j,k;var temp_r=0,temp_b=0,temp_br=0,temp_tr=0;for(i=0;i<6;i++){for(j=0;j<7;j++){temp_r=0;temp_b=0;temp_br=0;temp_tr=0;for(k=0;k<=3;k++){if(j+k<7){temp_r+=this.map[i][j+k];}if(i+k<6){temp_b+=this.map[i+k][j];}if(i+k<6&&j+k<7){temp_br+=this.map[i+k][j+k];}if(i-k>=0&&j+k<7){temp_tr+=this.map[i-k][j+k];}}if(Math.abs(temp_r)===4){this.win(temp_r);}else if(Math.abs(temp_b)===4){this.win(temp_b);}else if(Math.abs(temp_br)===4){this.win(temp_br);}else if(Math.abs(temp_tr)===4){this.win(temp_tr);}}}if((this.move===42)&&(! this.won)){this.win(0);}};this.drawCircle=function(x,y,r,fill,stroke){this.context.save();this.context.fillStyle=fill;this.context.strokeStyle=stroke;this.context.beginPath();this.context.arc(x,y,r,0,2*Math.PI,false);this.context.fill();this.context.restore();};this.drawMask=function(){this.context.save();this.context.fillStyle="#00bfff";this.context.beginPath();var x,y;for(y=0;y<6;y++){for(x=0;x<7;x++){this.context.arc(75*x+100,75*y+50,25,0,2*Math.PI);this.context.rect(75*x+150,75*y,-100,100);}}this.context.fill();this.context.restore();};this.draw=function(){var x,y;var fg_color;for(y=0;y<6;y++){for(x=0;x<7;x++){fg_color="transparent";if(this.map[y][x]>=1){fg_color="#ff4136";}else if(this.map[y][x]<=-1){fg_color="#ffff00";}this.drawCircle(75*x+100,75*y+50,25,fg_color,"black");}}};this.clear=function(){this.context.clearRect(0,0,this.canvas.width,this.canvas.height);};this.animate=function(column,move,to_row,cur_pos,callback){var fg_color="transparent";if(move>=1){fg_color="#ff4136";}else if(move<=-1){fg_color="#ffff00";}if(to_row*75>=cur_pos){this.clear();this.draw();this.drawCircle(75*column+100,cur_pos+50,25,fg_color,"black");this.drawMask();window.requestAnimationFrame(function(){that.animate(column,move,to_row,cur_pos+25,callback);});}else{callback();}};this.onregion=function(coord,x,radius){if((coord[0]-x)*(coord[0]-x)<=radius*radius){return true;}return false;};this.oncircle=function(coord,centerCoord,radius){if((coord[0]-centerCoord[0])*(coord[0]-centerCoord[0])<=radius*radius&&(coord[1]-centerCoord[1])*(coord[1]-centerCoord[1])<=radius*radius){return true;}return false;};this.onclick=function(canvas,e){if(this.rejectClick){return false;}if(this.won){this.init();return false;}var rect=canvas.getBoundingClientRect(),x=e.clientX-rect.left,y=e.clientY-rect.top;var j,valid;for(j=0;j<7;j++){if(this.onregion([x,y],75*j+100,25)){this.paused=false;this.action(j);if(valid===1){this.rejectClick=true;}break;}}};this.init();S_scope.games.push(S_scope.newGame);};}})());
            },
            "__cargo_web_snippet_a5635a9b99d3b34c4ee6acd8671adae22c29f80d": function($0) {
                Module.STDWEB_PRIVATE.from_js($0, (function(){angular.module("Connect4App").controller("tootHumanController",tootHumanController);app.factory("postService",function(S_resource){return S_resource("/games");});function tootHumanController(postService,S_scope,S_rootScope){S_scope.games=[];S_scope.newGame={gameNumber:"100",gameType:"TOOT-OTTO",Player1Name:"",Player2Name:"",WinnerName:"Unfinished Game",GameDate:Date.now(),Label:""};Array.prototype.clone=function(){var arr=[],i;for(i=0;i<this.length;i++){arr[i]=this[i].slice();}return arr;};S_scope.Game=function(){var target=document.getElementById("gameboard");var button=document.getElementById("startbutton");button.disabled=true;var target1=document.getElementById("textbox1");var target2=document.getElementById("textbox2");var that=this;this.map=[];this.dummyMap=[];this.paused=false;this.won=false;this.rejectClick=false;this.move=0;this.aiHistory=[];this.initOnceDone=false;this.initOnce=function(){if(this.initOnceDone){return false;}this.canvas=document.getElementsByTagName("canvas")[0];this.canvas.addEventListener("click",function(e){if(S_scope.newGame.Label===""){return false;}that.onclick(that.canvas,e);});this.context=this.canvas.getContext("2d");this.context.font="bold 25px serif";this.initOnceDone=true;};this.init=function(){this.map=[];this.dummyMap=[];this.paused=false;this.won=false;this.rejectClick=false;this.move=0;this.aiHistory=[];this.initOnce();var i,j;for(i=0;i<=6;i++){this.map[i]=[];this.dummyMap[i]=[];for(j=0;j<=7;j++){this.map[i][j]=0;this.dummyMap[i][j]=0;}}this.clear();this.drawMask();this.print();};this.playerMove=function(){if(this.move%2===0){return 1;}return-1;};this.print=function(){var i,j,msg,dummymsg="";msg="\n";msg+="Move: "+this.move;msg+="\n";for(i=0;i<6;i++){for(j=0;j<7;j++){msg+=" "+this.map[i][j];dummymsg+=" "+this.dummyMap[i][j];}msg+="\n";dummymsg+="\n";}console.log(msg);console.log(dummymsg);};this.printState=function(state){var i,j,msg="\n";for(i=0;i<6;i++){for(j=0;j<7;j++){msg+=" "+state[i][j];}msg+="\n";}console.log(msg);};this.win=function(player){this.paused=true;this.won=true;this.rejectClick=false;var msg=null;if(player>0){msg=S_scope.newGame.Player1Name+" wins";S_scope.newGame.WinnerName=S_scope.newGame.Player1Name;}else if(player<0){msg=S_scope.newGame.Player2Name+" wins";S_scope.newGame.WinnerName=S_scope.newGame.Player2Name;}else{msg="It's a draw";S_scope.newGame.WinnerName="Draw";}msg+=" - Click on game board to reset";this.context.save();this.context.font="14pt sans-serif";this.context.fillStyle="#111";this.context.fillText(msg,150,20);postService.save(S_scope.newGame,function(){console.log("succesfully saved");});this.canvas=document.getElementsByTagName("canvas")[0];this.canvas.addEventListener("click",function(e){location.reload();});button.disabled=false;console.info(msg);};this.fillMap=function(state,column,value){var tempMap=state.clone();if(tempMap[0][column]!==0||column<0||column>6){return-1;}var done=false,row=0,i;for(i=0;i<5;i++){if(tempMap[i+1][column]!==0){done=true;row=i;break;}}if(! done){row=5;}tempMap[row][column]=value;return tempMap;};this.action=function(column,callback){if(this.paused||this.won){return 0;}if(this.map[0][column]!==0||column<0||column>6){return-1;}var done=false;var row=0,i;for(i=0;i<5;i++){if(this.map[i+1][column]!==0){done=true;row=i;break;}}if(! done){row=5;}this.animate(column,this.playerMove(this.move),row,0,function(){that.map[row][column]=that.playerMove(that.move);that.dummyMap[row][column]=S_scope.newGame.Label;that.move++;that.draw();that.check();that.print();});this.paused=true;return 1;};this.check=function(){var i,j,k;var temp_r=0,temp_b=0,temp_br=0,temp_tr=0;var temp_r1=[],temp_b1=[],temp_br1=[],temp_br2=[];for(i=0;i<6;i++){for(j=0;j<7;j++){temp_r1[0]=0;temp_r1[1]=0;temp_r1[2]=0;temp_r1[3]=0;temp_b1[0]=0;temp_b1[1]=0;temp_b1[2]=0;temp_b1[3]=0;temp_br1[0]=0;temp_br1[1]=0;temp_br1[2]=0;temp_br1[3]=0;temp_br2[0]=0;temp_br2[1]=0;temp_br2[2]=0;temp_br2[3]=0;for(k=0;k<=3;k++){if(j+k<7){temp_r1[k]=this.dummyMap[i][j+k];}if(i+k<6){temp_b1[k]=this.dummyMap[i+k][j];}if(i+k<6&&j+k<7){temp_br1[k]=this.dummyMap[i+k][j+k];}if(i-k>=0&&j+k<7){temp_br2[k]=this.dummyMap[i-k][j+k];}}if(temp_r1[0]==="T"&&temp_r1[1]==="O"&&temp_r1[2]==="O"&&temp_r1[3]==="T"){this.win(1);}else if(temp_r1[0]==="O"&&temp_r1[1]==="T"&&temp_r1[2]==="T"&&temp_r1[3]==="O"){this.win(-1);}else if(temp_b1[0]==="T"&&temp_b1[1]==="O"&&temp_b1[2]==="O"&&temp_b1[3]==="T"){this.win(1);}else if(temp_b1[0]==="O"&&temp_b1[1]==="T"&&temp_b1[2]==="T"&&temp_b1[3]==="O"){this.win(-1);}else if(temp_br1[0]==="T"&&temp_br1[1]==="O"&&temp_br1[2]==="O"&&temp_br1[3]==="T"){this.win(1);}else if(temp_br1[0]==="O"&&temp_br1[1]==="T"&&temp_br1[2]==="T"&&temp_br1[3]==="O"){this.win(-1);}else if(temp_br2[0]==="T"&&temp_br2[1]==="O"&&temp_br2[2]==="O"&&temp_br2[3]==="T"){this.win(1);}else if(temp_br2[0]==="O"&&temp_br2[1]==="T"&&temp_br2[2]==="T"&&temp_br2[3]==="O"){this.win(-1);}}}if((this.move===42)&&(! this.won)){this.win(0);}};this.drawCircle=function(x,y,r,fill,stroke,text){this.context.save();this.context.fillStyle=fill;this.context.strokeStyle=stroke;this.context.beginPath();this.context.arc(x,y,r,0,2*Math.PI,false);this.context.fill();this.context.font="bold 25px serif";this.context.restore();this.context.fillText(text,x-8.5,y+8);};this.drawMask=function(){this.context.save();this.context.fillStyle="#00bfff";this.context.beginPath();var x,y;for(y=0;y<6;y++){for(x=0;x<7;x++){this.context.arc(75*x+100,75*y+50,25,0,2*Math.PI);this.context.rect(75*x+150,75*y,-100,100);}}this.context.fill();this.context.restore();};this.draw=function(){var x,y;var fg_color;for(y=0;y<6;y++){for(x=0;x<7;x++){var text="";fg_color="transparent";if(this.map[y][x]>=1&&this.dummyMap[y][x]==="T"){fg_color="#99ffcc";text="T";}else if(this.map[y][x]>=1&&this.dummyMap[y][x]==="O"){fg_color="#99ffcc";text="O";}else if(this.map[y][x]<=-1&&this.dummyMap[y][x]==="T"){fg_color="#ffff99";text="T";}else if(this.map[y][x]<=-1&&this.dummyMap[y][x]==="O"){fg_color="#ffff99";text="O";}this.drawCircle(75*x+100,75*y+50,25,fg_color,"black",text);}}};this.clear=function(){this.context.clearRect(0,0,this.canvas.width,this.canvas.height);};this.animate=function(column,move,to_row,cur_pos,callback){var text="";var fg_color="transparent";if(move>=1){fg_color="#99ffcc";text=S_scope.newGame.Label;}else if(move<=-1){fg_color="#ffff99";text=S_scope.newGame.Label;}if(to_row*75>=cur_pos){this.clear();this.draw();this.drawCircle(75*column+100,cur_pos+50,25,fg_color,"black",text);this.drawMask();window.requestAnimationFrame(function(){that.animate(column,move,to_row,cur_pos+25,callback);});}else{callback();}};this.onregion=function(coord,x,radius){if((coord[0]-x)*(coord[0]-x)<=radius*radius){return true;}return false;};this.oncircle=function(coord,centerCoord,radius){if((coord[0]-centerCoord[0])*(coord[0]-centerCoord[0])<=radius*radius&&(coord[1]-centerCoord[1])*(coord[1]-centerCoord[1])<=radius*radius){return true;}return false;};this.onclick=function(canvas,e){if(this.rejectClick){return false;}if(this.won){this.init();return false;}var rect=canvas.getBoundingClientRect(),x=e.clientX-rect.left,y=e.clientY-rect.top;var j,valid;for(j=0;j<7;j++){if(this.onregion([x,y],75*j+100,25)){this.paused=false;this.action(j);if(valid===1){this.rejectClick=true;}break;}}};this.init();S_scope.games.push(S_scope.newGame);};}})());
            },
            "__cargo_web_snippet_aff9f805a5772de4e5c42be7640fceff4d159e32": function($0) {
                Module.STDWEB_PRIVATE.from_js($0, (function(){angular.module("Connect4App").controller("tootComputerController",tootComputerController);app.factory("postService",function(S_resource){return S_resource("/games");});function tootComputerController(postService,S_scope,S_rootScope){S_scope.games=[];S_scope.newGame={gameNumber:"100",gameType:"TOOT-OTTO",Player1Name:"",Player2Name:"Computer",WinnerName:"Unfinished Game",GameDate:Date.now(),Label:""};Array.prototype.clone=function(){var arr=[],i;for(i=0;i<this.length;i++){arr[i]=this[i].slice();}return arr;};S_scope.Game=function(){var target=document.getElementById("gameboard");var button=document.getElementById("startbutton");button.disabled=true;var target1=document.getElementById("textbox1");var that=this;this.map=[];this.dummyMap=[];this.paused=false;this.won=false;this.rejectClick=false;this.move=0;this.aiHistory=[];this.initOnceDone=false;this.initOnce=function(){if(this.initOnceDone){return false;}this.canvas=document.getElementsByTagName("canvas")[0];this.canvas.addEventListener("click",function(e){if(S_scope.newGame.Label===""){return false;}that.onclick(that.canvas,e);});this.context=this.canvas.getContext("2d");this.context.font="bold 25px serif";this.initOnceDone=true;};this.init=function(){this.map=[];this.dummyMap=[];this.paused=false;this.won=false;this.rejectClick=false;this.move=0;this.aiHistory=[];this.initOnce();var i,j;for(i=0;i<=6;i++){this.map[i]=[];this.dummyMap[i]=[];for(j=0;j<=7;j++){this.map[i][j]=0;this.dummyMap[i][j]=0;}}this.clear();this.drawMask();this.print();};this.playerMove=function(){if(this.move%2===0){return 1;}return-1;};this.print=function(){var i,j,msg;msg="\n";msg+="Move: "+this.move;msg+="\n";for(i=0;i<6;i++){for(j=0;j<7;j++){msg+=" "+this.map[i][j];}msg+="\n";}console.log(msg);};this.printState=function(state){var i,j,msg="\n";for(i=0;i<6;i++){for(j=0;j<7;j++){msg+=" "+state[i][j];}msg+="\n";}console.log(msg);};this.win=function(player){this.paused=true;this.won=true;this.rejectClick=false;var msg=null;if(player>0){msg=S_scope.newGame.Player1Name+" wins";S_scope.newGame.WinnerName=S_scope.newGame.Player1Name;}else if(player<0){msg=S_scope.newGame.Player2Name+" wins";S_scope.newGame.WinnerName=S_scope.newGame.Player2Name;}else{msg="It's a draw";S_scope.newGame.WinnerName="Draw";}msg+=" - Click on game board to reset";this.context.save();this.context.font="14pt sans-serif";this.context.fillStyle="#111";this.context.fillText(msg,150,20);this.context.restore();postService.save(S_scope.newGame,function(){console.log("succesfully saved");});this.canvas=document.getElementsByTagName("canvas")[0];this.canvas.addEventListener("click",function(e){location.reload();});button.disabled=false;console.info(msg);};this.fillMap=function(state,column,value){var tempMap=state.clone();if(tempMap[0][column]!==0||column<0||column>6){return-1;}var done=false,row=0,i;for(i=0;i<5;i++){if(tempMap[i+1][column]!==0){done=true;row=i;break;}}if(! done){row=5;}tempMap[row][column]=value;return tempMap;};this.action=function(column,callback){if(this.paused||this.won){return 0;}if(this.map[0][column]!==0||column<0||column>6){return-1;}var done=false;var row=0,i;for(i=0;i<5;i++){if(this.map[i+1][column]!==0){done=true;row=i;break;}}if(! done){row=5;}this.animate(column,this.playerMove(this.move),row,0,function(){that.map[row][column]=that.playerMove(that.move);that.dummyMap[row][column]=S_scope.newGame.Label;that.move++;that.draw();that.check();that.print();callback();});this.paused=true;return 1;};this.check=function(){var i,j,k;var temp_r=0,temp_b=0,temp_br=0,temp_tr=0;var temp_r1=[],temp_b1=[],temp_br1=[],temp_br2=[];for(i=0;i<6;i++){for(j=0;j<7;j++){temp_r1[0]=0;temp_r1[1]=0;temp_r1[2]=0;temp_r1[3]=0;temp_b1[0]=0;temp_b1[1]=0;temp_b1[2]=0;temp_b1[3]=0;temp_br1[0]=0;temp_br1[1]=0;temp_br1[2]=0;temp_br1[3]=0;temp_br2[0]=0;temp_br2[1]=0;temp_br2[2]=0;temp_br2[3]=0;for(k=0;k<=3;k++){if(j+k<7){temp_r1[k]=this.dummyMap[i][j+k];}if(i+k<6){temp_b1[k]=this.dummyMap[i+k][j];}if(i+k<6&&j+k<7){temp_br1[k]=this.dummyMap[i+k][j+k];}if(i-k>=0&&j+k<7){temp_br2[k]=this.dummyMap[i-k][j+k];}}if(temp_r1[0]==="T"&&temp_r1[1]==="O"&&temp_r1[2]==="O"&&temp_r1[3]==="T"){this.win(1);}else if(temp_r1[0]==="O"&&temp_r1[1]==="T"&&temp_r1[2]==="T"&&temp_r1[3]==="O"){this.win(-1);}else if(temp_b1[0]==="T"&&temp_b1[1]==="O"&&temp_b1[2]==="O"&&temp_b1[3]==="T"){this.win(1);}else if(temp_b1[0]==="O"&&temp_b1[1]==="T"&&temp_b1[2]==="T"&&temp_b1[3]==="O"){this.win(-1);}else if(temp_br1[0]==="T"&&temp_br1[1]==="O"&&temp_br1[2]==="O"&&temp_br1[3]==="T"){this.win(1);}else if(temp_br1[0]==="O"&&temp_br1[1]==="T"&&temp_br1[2]==="T"&&temp_br1[3]==="O"){this.win(-1);}else if(temp_br2[0]==="T"&&temp_br2[1]==="O"&&temp_br2[2]==="O"&&temp_br2[3]==="T"){this.win(1);}else if(temp_br2[0]==="O"&&temp_br2[1]==="T"&&temp_br2[2]==="T"&&temp_br2[3]==="O"){this.win(-1);}}}if((this.move===42)&&(! this.won)){this.win(0);}};this.drawCircle=function(x,y,r,fill,stroke,text){this.context.save();this.context.fillStyle=fill;this.context.strokeStyle=stroke;this.context.beginPath();this.context.arc(x,y,r,0,2*Math.PI,false);this.context.fill();this.context.font="bold 25px serif";this.context.restore();this.context.fillText(text,x-8.5,y+8);};this.drawMask=function(){this.context.save();this.context.fillStyle="#00bfff";this.context.beginPath();var x,y;for(y=0;y<6;y++){for(x=0;x<7;x++){this.context.arc(75*x+100,75*y+50,25,0,2*Math.PI);this.context.rect(75*x+150,75*y,-100,100);}}this.context.fill();this.context.restore();};this.draw=function(){var x,y;var fg_color;for(y=0;y<6;y++){for(x=0;x<7;x++){var text="";fg_color="transparent";if(this.map[y][x]>=1&&this.dummyMap[y][x]==="T"){fg_color="#99ffcc";text="T";}else if(this.map[y][x]>=1&&this.dummyMap[y][x]==="O"){fg_color="#99ffcc";text="O";}else if(this.map[y][x]<=-1&&this.dummyMap[y][x]==="T"){fg_color="#ffff99";text="T";}else if(this.map[y][x]<=-1&&this.dummyMap[y][x]==="O"){fg_color="#ffff99";text="O";}this.drawCircle(75*x+100,75*y+50,25,fg_color,"black",text);}}};this.clear=function(){this.context.clearRect(0,0,this.canvas.width,this.canvas.height);};this.animate=function(column,move,to_row,cur_pos,callback){var text="";var fg_color="transparent";if(move>=1){fg_color="#99ffcc";text=S_scope.newGame.Label;}else if(move<=-1){fg_color="#ffff99";text=S_scope.newGame.Label;}if(to_row*75>=cur_pos){this.clear();this.draw();this.drawCircle(75*column+100,cur_pos+50,25,fg_color,"black",text);this.drawMask();window.requestAnimationFrame(function(){that.animate(column,move,to_row,cur_pos+25,callback);});}else{callback();}};this.onregion=function(coord,x,radius){if((coord[0]-x)*(coord[0]-x)<=radius*radius){return true;}return false;};this.oncircle=function(coord,centerCoord,radius){if((coord[0]-centerCoord[0])*(coord[0]-centerCoord[0])<=radius*radius&&(coord[1]-centerCoord[1])*(coord[1]-centerCoord[1])<=radius*radius){return true;}return false;};this.onclick=function(canvas,e){if(this.rejectClick){return false;}if(this.won){this.init();return false;}var rect=canvas.getBoundingClientRect(),x=e.clientX-rect.left,y=e.clientY-rect.top;var j,valid;for(j=0;j<7;j++){if(this.onregion([x,y],75*j+100,25)){this.paused=false;valid=this.action(j,function(){that.ai(-1);});if(valid===1){this.rejectClick=true;}break;}}};this.ai=function(aiMoveValue){var dummyLabel=[];dummyLabel[0]="T";dummyLabel[1]="O";var choice=Math.floor(Math.random()*2);S_scope.newGame.Label=dummyLabel[choice];var choice=null;var state=this.map.clone();function checkState(state){var winVal=0;var chainVal=0;var i,j,k;var temp_r=0,temp_b=0,temp_br=0,temp_tr=0;for(i=0;i<6;i++){for(j=0;j<7;j++){temp_r=0;temp_b=0;temp_br=0;temp_tr=0;for(k=0;k<=3;k++){if(j+k<7){temp_r+=state[i][j+k];}if(i+k<6){temp_b+=state[i+k][j];}if(i+k<6&&j+k<7){temp_br+=state[i+k][j+k];}if(i-k>=0&&j+k<7){temp_tr+=state[i-k][j+k];}}chainVal+=temp_r*temp_r*temp_r;chainVal+=temp_b*temp_b*temp_b;chainVal+=temp_br*temp_br*temp_br;chainVal+=temp_tr*temp_tr*temp_tr;if(Math.abs(temp_r)===4){winVal=temp_r;}else if(Math.abs(temp_b)===4){winVal=temp_b;}else if(Math.abs(temp_br)===4){winVal=temp_br;}else if(Math.abs(temp_tr)===4){winVal=temp_tr;}}}return[winVal,chainVal];}function value(state,depth,alpha,beta){var val=checkState(state);if(depth>=4){var retValue=0;var winVal=val[0];var chainVal=val[1]*aiMoveValue;retValue=chainVal;if(winVal===4*aiMoveValue){retValue=999999;}else if(winVal===4*aiMoveValue*-1){retValue=999999*-1;}retValue-=depth*depth;return[retValue,-1];}var win=val[0];if(win===4*aiMoveValue){return[999999-depth*depth,-1];}if(win===4*aiMoveValue*-1){return[999999*-1-depth*depth,-1];}if(depth%2===0){return minState(state,depth+1,alpha,beta);}return maxState(state,depth+1,alpha,beta);}function choose(choice){return choice[Math.floor(Math.random()*choice.length)];}function maxState(state,depth,alpha,beta){var v=-100000000007;var move=-1;var tempVal=null;var tempState=null;var moveQueue=[];var j;for(j=0;j<7;j++){tempState=that.fillMap(state,j,aiMoveValue);if(tempState !==-1){tempVal=value(tempState,depth,alpha,beta);if(tempVal[0]>v){v=tempVal[0];move=j;moveQueue=[];moveQueue.push(j);}else if(tempVal[0]===v){moveQueue.push(j);}if(v>beta){move=choose(moveQueue);return[v,move];}alpha=Math.max(alpha,v);}}move=choose(moveQueue);return[v,move];}function minState(state,depth,alpha,beta){var v=100000000007;var move=-1;var tempVal=null;var tempState=null;var moveQueue=[];var j;for(j=0;j<7;j++){tempState=that.fillMap(state,j,aiMoveValue*-1);if(tempState !==-1){tempVal=value(tempState,depth,alpha,beta);if(tempVal[0]<v){v=tempVal[0];move=j;moveQueue=[];moveQueue.push(j);}else if(tempVal[0]===v){moveQueue.push(j);}if(v<alpha){move=choose(moveQueue);return[v,move];}beta=Math.min(beta,v);}}move=choose(moveQueue);return[v,move];}var choice_val=maxState(state,0,-100000000007,100000000007);choice=choice_val[1];var val=choice_val[0];console.info("AI "+aiMoveValue+" choose column: "+choice+" (value: "+val+")");this.paused=false;var done=this.action(choice,function(){that.rejectClick=false;});while(done<0){console.error("Falling back to random agent");choice=Math.floor(Math.random()*7);done=this.action(choice,function(){that.rejectClick=false;});}};this.init();S_scope.games.push(S_scope.newGame);};}})());
            },
            "__cargo_web_snippet_d4cd475855d07f8f066c9610b6de97ccd48d16fd": function($0) {
                Module.STDWEB_PRIVATE.from_js($0, (function(){var app=angular.module("Connect4App",["ngRoute","ngResource","angular.filter"]);app.config(function(S_routeProvider){S_routeProvider.when("/",{templateUrl:"main.html",controller:"mainController"}).when("/Connect4Computer",{templateUrl:"Connect4Computer.html",controller:"mainController"}).when("/ScoreBoard",{templateUrl:"ScoreBoard.html",controller:"ScoreBoardCtrl"}).when("/Scores",{templateUrl:"Scores.html",controller:"ScoreBoardCtrl"}).when("/TootOttoHuman",{templateUrl:"TootOttoHuman.html",controller:"tootHumanController"}).when("/TootOttoComputer",{templateUrl:"TootOttoComputer.html",controller:"tootComputerController"}).when("/HowToToot",{templateUrl:"HowToToot.html",controller:"mainController"}).when("/HowToConnect4",{templateUrl:"HowToConnect4.html",controller:"mainController"}).when("/Connect4Human",{templateUrl:"Connect4Human.html",controller:"humanController"});});app.factory("postService",function(S_resource){return S_resource("/games");});app.controller("mainController",function(postService,S_scope,S_rootScope){S_scope.games=[];S_scope.newGame={gameNumber:"100",gameType:"Connect-4",Player1Name:"",Player2Name:"Computer",WinnerName:"",GameDate:Date.now()};Array.prototype.clone=function(){var arr=[],i;for(i=0;i<this.length;i++){arr[i]=this[i].slice();}return arr;};S_scope.Game=function(){var target=document.getElementById("gameboard");var button=document.getElementById("startbutton");button.disabled=true;var target1=document.getElementById("textbox1");var that=this;this.map=[];this.paused=false;this.won=false;this.rejectClick=false;this.move=0;this.aiHistory=[];this.initOnceDone=false;this.initOnce=function(){if(this.initOnceDone){return false;}this.canvas=document.getElementsByTagName("canvas")[0];this.canvas.addEventListener("click",function(e){that.onclick(that.canvas,e);});this.context=this.canvas.getContext("2d");this.initOnceDone=true;};this.init=function(){this.map=[];this.paused=false;this.won=false;this.rejectClick=false;this.move=0;this.aiHistory=[];this.initOnce();var i,j;for(i=0;i<=6;i++){this.map[i]=[];for(j=0;j<=7;j++){this.map[i][j]=0;}}this.clear();this.drawMask();this.print();};this.playerMove=function(){if(this.move%2===0){return 1;}return-1;};this.print=function(){var i,j,msg;msg="\n";msg+="Move: "+this.move;msg+="\n";for(i=0;i<6;i++){for(j=0;j<7;j++){msg+=" "+this.map[i][j];}msg+="\n";}console.log(msg);};this.printState=function(state){var i,j,msg="\n";for(i=0;i<6;i++){for(j=0;j<7;j++){msg+=" "+state[i][j];}msg+="\n";}console.log(msg);};this.win=function(player){this.paused=true;this.won=true;this.rejectClick=false;var msg=null;if(player>0){msg=S_scope.newGame.Player1Name+" wins";S_scope.newGame.WinnerName=S_scope.newGame.Player1Name;}else if(player<0){msg=S_scope.newGame.Player2Name+" wins";S_scope.newGame.WinnerName=S_scope.newGame.Player2Name;}else{msg="It's a draw";S_scope.newGame.WinnerName="Draw";}msg+=" - Click on game board to reset";this.context.save();this.context.font="14pt sans-serif";this.context.fillStyle="#111";this.context.fillText(msg,130,20);this.context.restore();postService.save(S_scope.newGame,function(){console.log("succesfully saved");});this.canvas=document.getElementsByTagName("canvas")[0];this.canvas.addEventListener("click",function(e){location.reload();});button.disabled=false;console.info(msg);};this.fillMap=function(state,column,value){var tempMap=state.clone();if(tempMap[0][column]!==0||column<0||column>6){return-1;}var done=false,row=0,i;for(i=0;i<5;i++){if(tempMap[i+1][column]!==0){done=true;row=i;break;}}if(! done){row=5;}tempMap[row][column]=value;return tempMap;};this.action=function(column,callback){if(this.paused||this.won){return 0;}if(this.map[0][column]!==0||column<0||column>6){return-1;}var done=false;var row=0,i;for(i=0;i<5;i++){if(this.map[i+1][column]!==0){done=true;row=i;break;}}if(! done){row=5;}this.animate(column,this.playerMove(this.move),row,0,function(){that.map[row][column]=that.playerMove(that.move);that.move++;that.draw();that.check();that.print();callback();});this.paused=true;return 1;};this.check=function(){var i,j,k;var temp_r=0,temp_b=0,temp_br=0,temp_tr=0;for(i=0;i<6;i++){for(j=0;j<7;j++){temp_r=0;temp_b=0;temp_br=0;temp_tr=0;for(k=0;k<=3;k++){if(j+k<7){temp_r+=this.map[i][j+k];}if(i+k<6){temp_b+=this.map[i+k][j];}if(i+k<6&&j+k<7){temp_br+=this.map[i+k][j+k];}if(i-k>=0&&j+k<7){temp_tr+=this.map[i-k][j+k];}}if(Math.abs(temp_r)===4){this.win(temp_r);}else if(Math.abs(temp_b)===4){this.win(temp_b);}else if(Math.abs(temp_br)===4){this.win(temp_br);}else if(Math.abs(temp_tr)===4){this.win(temp_tr);}}}if((this.move===42)&&(! this.won)){this.win(0);}};this.drawCircle=function(x,y,r,fill,stroke){this.context.save();this.context.fillStyle=fill;this.context.strokeStyle=stroke;this.context.beginPath();this.context.arc(x,y,r,0,2*Math.PI,false);this.context.fill();this.context.restore();};this.drawMask=function(){this.context.save();this.context.fillStyle="#00bfff";this.context.beginPath();var x,y;for(y=0;y<6;y++){for(x=0;x<7;x++){this.context.arc(75*x+100,75*y+50,25,0,2*Math.PI);this.context.rect(75*x+150,75*y,-100,100);}}this.context.fill();this.context.restore();};this.draw=function(){var x,y;var fg_color;for(y=0;y<6;y++){for(x=0;x<7;x++){fg_color="transparent";if(this.map[y][x]>=1){fg_color="#ff4136";}else if(this.map[y][x]<=-1){fg_color="#ffff00";}this.drawCircle(75*x+100,75*y+50,25,fg_color,"black");}}};this.clear=function(){this.context.clearRect(0,0,this.canvas.width,this.canvas.height);};this.animate=function(column,move,to_row,cur_pos,callback){var fg_color="transparent";if(move>=1){fg_color="#ff4136";}else if(move<=-1){fg_color="#ffff00";}if(to_row*75>=cur_pos){this.clear();this.draw();this.drawCircle(75*column+100,cur_pos+50,25,fg_color,"black");this.drawMask();window.requestAnimationFrame(function(){that.animate(column,move,to_row,cur_pos+25,callback);});}else{callback();}};this.onregion=function(coord,x,radius){if((coord[0]-x)*(coord[0]-x)<=radius*radius){return true;}return false;};this.oncircle=function(coord,centerCoord,radius){if((coord[0]-centerCoord[0])*(coord[0]-centerCoord[0])<=radius*radius&&(coord[1]-centerCoord[1])*(coord[1]-centerCoord[1])<=radius*radius){return true;}return false;};this.onclick=function(canvas,e){if(this.rejectClick){return false;}if(this.won){this.init();return false;}var rect=canvas.getBoundingClientRect(),x=e.clientX-rect.left,y=e.clientY-rect.top;var j,valid;for(j=0;j<7;j++){if(this.onregion([x,y],75*j+100,25)){this.paused=false;valid=this.action(j,function(){that.ai(-1);});if(valid===1){this.rejectClick=true;}break;}}};this.ai=function(aiMoveValue){var choice=null;var state=this.map.clone();function checkState(state){var winVal=0;var chainVal=0;var i,j,k;var temp_r=0,temp_b=0,temp_br=0,temp_tr=0;for(i=0;i<6;i++){for(j=0;j<7;j++){temp_r=0;temp_b=0;temp_br=0;temp_tr=0;for(k=0;k<=3;k++){if(j+k<7){temp_r+=state[i][j+k];}if(i+k<6){temp_b+=state[i+k][j];}if(i+k<6&&j+k<7){temp_br+=state[i+k][j+k];}if(i-k>=0&&j+k<7){temp_tr+=state[i-k][j+k];}}chainVal+=temp_r*temp_r*temp_r;chainVal+=temp_b*temp_b*temp_b;chainVal+=temp_br*temp_br*temp_br;chainVal+=temp_tr*temp_tr*temp_tr;if(Math.abs(temp_r)===4){winVal=temp_r;}else if(Math.abs(temp_b)===4){winVal=temp_b;}else if(Math.abs(temp_br)===4){winVal=temp_br;}else if(Math.abs(temp_tr)===4){winVal=temp_tr;}}}return[winVal,chainVal];}function value(state,depth,alpha,beta){var val=checkState(state);if(depth>=4){var retValue=0;var winVal=val[0];var chainVal=val[1]*aiMoveValue;retValue=chainVal;if(winVal===4*aiMoveValue){retValue=999999;}else if(winVal===4*aiMoveValue*-1){retValue=999999*-1;}retValue-=depth*depth;return[retValue,-1];}var win=val[0];if(win===4*aiMoveValue){return[999999-depth*depth,-1];}if(win===4*aiMoveValue*-1){return[999999*-1-depth*depth,-1];}if(depth%2===0){return minState(state,depth+1,alpha,beta);}return maxState(state,depth+1,alpha,beta);}function choose(choice){return choice[Math.floor(Math.random()*choice.length)];}function maxState(state,depth,alpha,beta){var v=-100000000007;var move=-1;var tempVal=null;var tempState=null;var moveQueue=[];var j;for(j=0;j<7;j++){tempState=that.fillMap(state,j,aiMoveValue);if(tempState !==-1){tempVal=value(tempState,depth,alpha,beta);if(tempVal[0]>v){v=tempVal[0];move=j;moveQueue=[];moveQueue.push(j);}else if(tempVal[0]===v){moveQueue.push(j);}if(v>beta){move=choose(moveQueue);return[v,move];}alpha=Math.max(alpha,v);}}move=choose(moveQueue);return[v,move];}function minState(state,depth,alpha,beta){var v=100000000007;var move=-1;var tempVal=null;var tempState=null;var moveQueue=[];var j;for(j=0;j<7;j++){tempState=that.fillMap(state,j,aiMoveValue*-1);if(tempState !==-1){tempVal=value(tempState,depth,alpha,beta);if(tempVal[0]<v){v=tempVal[0];move=j;moveQueue=[];moveQueue.push(j);}else if(tempVal[0]===v){moveQueue.push(j);}if(v<alpha){move=choose(moveQueue);return[v,move];}beta=Math.min(beta,v);}}move=choose(moveQueue);return[v,move];}var choice_val=maxState(state,0,-100000000007,100000000007);choice=choice_val[1];var val=choice_val[0];console.info("AI "+aiMoveValue+" choose column: "+choice+" (value: "+val+")");this.paused=false;var done=this.action(choice,function(){that.rejectClick=false;});while(done<0){console.error("Falling back to random agent");choice=Math.floor(Math.random()*7);done=this.action(choice,function(){that.rejectClick=false;});}};this.init();S_scope.games.push(S_scope.newGame);};});})());
            },
            "__cargo_web_snippet_e9638d6405ab65f78daf4a5af9c9de14ecf1e2ec": function($0) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);Module.STDWEB_PRIVATE.unregister_raw_value(($0));
            },
                "__web_on_grow": __web_on_grow
            }
        },
        initialize: function( instance ) {
            Object.defineProperty( Module, 'instance', { value: instance } );
            Object.defineProperty( Module, 'web_malloc', { value: Module.instance.exports.__web_malloc } );
            Object.defineProperty( Module, 'web_free', { value: Module.instance.exports.__web_free } );
            Object.defineProperty( Module, 'web_table', { value: Module.instance.exports.__indirect_function_table } );

            
            __web_on_grow();
            Module.instance.exports.main();

            return Module.exports;
        }
    };
}
 ));
}));
