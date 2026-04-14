// yvals_core.h internal header (core)

// Copyright (c) Microsoft Corporation.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef _YVALS_CORE_H_
#define _YVALS_CORE_H_
#ifndef _STL_COMPILER_PREPROCESSOR
// All STL headers avoid exposing their contents when included by various
// non-C++-compiler tools to avoid breaking builds when we use newer language
// features in the headers than such tools understand.
#if defined(RC_INVOKED) || defined(Q_MOC_RUN) || defined(__midl)
#define _STL_COMPILER_PREPROCESSOR 0
#else
#define _STL_COMPILER_PREPROCESSOR 1
#endif
#endif // !defined(_STL_COMPILER_PREPROCESSOR)

#if _STL_COMPILER_PREPROCESSOR

// This does not use `_EMIT_STL_ERROR`, as it is checking the language itself.
#ifndef __cplusplus
#error error STL1003: Unexpected compiler, expected C++ compiler.
#endif // !defined(__cplusplus)

// Implemented unconditionally:
// N3911 void_t
// N4089 Safe Conversions In unique_ptr<T[]>
// N4169 invoke()
// N4258 noexcept Cleanups
// N4259 uncaught_exceptions()
// N4277 Trivially Copyable reference_wrapper
// N4279 insert_or_assign()/try_emplace() For map/unordered_map
// N4280 size(), empty(), data()
// N4366 Precisely Constraining unique_ptr Assignment
// N4387 Improving pair And tuple
// N4389 bool_constant
// N4508 shared_mutex (Untimed)
// N4510 Supporting Incomplete Types In vector/list/forward_list
// P0006R0 Variable Templates For Type Traits (is_same_v, etc.)
// P0007R1 as_const()
// P0013R1 Logical Operator Type Traits (conjunction, etc.)
// P0033R1 Rewording enable_shared_from_this
// P0063R3 C11 Standard Library
// P0074R0 owner_less<>
// P0092R1 <chrono> floor(), ceil(), round(), abs()
// P0340R3 SFINAE-Friendly underlying_type
// P0414R2 shared_ptr<T[]>, shared_ptr<T[N]>
// P0418R2 atomic compare_exchange memory_order Requirements
// P0435R1 Overhauling common_type
// P0497R0 Fixing shared_ptr For Arrays
// P0513R0 Poisoning hash
// P0516R0 Marking shared_future Copying As noexcept
// P0517R0 Constructing future_error From future_errc
// P0548R1 Tweaking common_type And duration
// P0558R1 Resolving atomic<T> Named Base Class Inconsistencies
// P0599R1 noexcept hash
// P0738R2 istream_iterator Cleanup
// P0771R1 noexcept For std::function's Move Constructor
// P0777R1 Avoiding Unnecessary decay
// P0809R0 Comparing Unordered Containers
// P0883R2 Fixing Atomic Initialization
// P0935R0 Eradicating Unnecessarily Explicit Default Constructors
// P0941R2 Feature-Test Macros
// P0952R2 A New Specification For generate_canonical()
// P0972R0 noexcept For <chrono> zero(), min(), max()
// P1065R2 constexpr INVOKE
//     (the std::invoke function only; other components like bind and reference_wrapper are C++20 only)
// P1164R1 Making create_directory() Intuitive
// P1165R1 Consistently Propagating Stateful Allocators In basic_string's operator+()
// P1902R1 Missing Feature-Test Macros 2017-2019
// P2013R5 Freestanding Language: Optional ::operator new
//     (no change is needed for our hosted implementation)
// P2198R7 Freestanding Feature-Test Macros And Implementation-Defined Extensions
//     (except for __cpp_lib_freestanding_ranges)
// P2338R4 Freestanding Library: Character Primitives And The C Library
//     (except for __cpp_lib_freestanding_charconv)
// P2401R0 Conditional noexcept For exchange()
// P2407R5 Freestanding Library: Partial Classes
//     (__cpp_lib_freestanding_algorithm and __cpp_lib_freestanding_array only)
// P2937R0 Freestanding Library: Remove strtok
// P2968R2 Make std::ignore A First-Class Object
// P3323R1 Forbid atomic<cv T>, Specify atomic_ref<cv T>
//     (for atomic<cv T>)

// _HAS_CXX17 directly controls:
// P0005R4 not_fn()
// P0024R2 Parallel Algorithms
// P0025R1 clamp()
// P0030R1 hypot(x, y, z)
// P0031R0 constexpr For <array> (Again) And <iterator>
// P0032R3 Homogeneous Interface For variant/any/optional
// P0040R3 Extending Memory Management Tools
// P0067R5 Elementary String Conversions
// P0083R3 Splicing Maps And Sets
// P0084R2 Emplace Return Type
// P0088R3 <variant>
// P0137R1 launder()
// P0152R1 atomic::is_always_lock_free
// P0154R1 hardware_destructive_interference_size, etc.
// P0156R2 scoped_lock
// P0163R0 shared_ptr::weak_type
// P0185R1 is_swappable, is_nothrow_swappable
// P0209R2 make_from_tuple()
// P0218R1 <filesystem>
// P0220R1 <any>, <memory_resource>, <optional>, <string_view>, apply(), sample(), Boyer-Moore search()
// P0226R1 Mathematical Special Functions
// P0253R1 Fixing Searcher Return Types
// P0254R2 Integrating string_view And std::string
// P0258R2 has_unique_object_representations
// P0272R1 Non-const basic_string::data()
// P0295R0 gcd(), lcm()
// P0307R2 Making Optional Greater Equal Again
// P0336R1 Renaming Parallel Execution Policies
// P0337R0 Deleting polymorphic_allocator Assignment
// P0358R1 Fixes For not_fn()
// P0393R3 Making Variant Greater Equal
// P0394R4 Parallel Algorithms Should terminate() For Exceptions
// P0403R1 UDLs For <string_view> ("meow"sv, etc.)
// P0426R1 constexpr For char_traits
// P0433R2 Deduction Guides For The STL
// P0452R1 Unifying <numeric> Parallel Algorithms
// P0504R0 Revisiting in_place_t/in_place_type_t<T>/in_place_index_t<I>
// P0505R0 constexpr For <chrono> (Again)
// P0508R0 Clarifying insert_return_type
// P0510R0 Rejecting variants Of Nothing, Arrays, References, And Incomplete Types
// P0602R4 Propagating Copy/Move Triviality In variant/optional
// P0604R0 invoke_result, is_invocable, is_nothrow_invocable
// P0607R0 Inline Variables For The STL
// P0608R3 Improving variant's Converting Constructor/Assignment
// P0682R1 Repairing Elementary String Conversions
// P0739R0 Improving Class Template Argument Deduction For The STL
// P0858R0 Constexpr Iterator Requirements
// P1169R4 static operator()
// P1518R2 Stop Overconstraining Allocators In Container Deduction Guides
// P2162R2 Inheriting From variant
// P2251R1 Require span And basic_string_view To Be Trivially Copyable
//     (basic_string_view always provides this behavior)
// P2338R4 Freestanding Library: Character Primitives And The C Library
//     (including __cpp_lib_freestanding_charconv)
// P2407R5 Freestanding Library: Partial Classes
//     (including __cpp_lib_freestanding_optional, __cpp_lib_freestanding_string_view, and
//     __cpp_lib_freestanding_variant)
// P2517R1 Conditional noexcept For apply()
// P2875R4 Undeprecate polymorphic_allocator::destroy

// _HAS_CXX17 indirectly controls:
// N4190 Removing auto_ptr, random_shuffle(), And Old <functional> Stuff
// P0003R5 Removing Dynamic Exception Specifications
// P0004R1 Removing Deprecated Iostreams Aliases
// P0298R3 std::byte
// P0302R1 Removing Allocator Support In std::function
// LWG-2385 function::assign allocator argument doesn't make sense
// LWG-2921 packaged_task and type-erased allocators
// LWG-2976 Dangling uses_allocator specialization for packaged_task
// The non-Standard std::tr1 namespace and TR1-only machinery
// Enforcement of matching allocator value_types

// _HAS_CXX17 and _SILENCE_ALL_CXX17_DEPRECATION_WARNINGS control:
// P0174R2 Deprecating Vestigial Library Parts
// P0521R0 Deprecating shared_ptr::unique()
// P0618R0 Deprecating <codecvt>
// Other C++17 deprecation warnings

// Implemented when char8_t is available (C++14/17 with /Zc:char8_t, C++20 without /Zc:char8_t-):
// P0482R6 Library Support For char8_t
//     (mbrtoc8 and c8rtomb not yet implemented, see GH-2207)

// _HAS_CXX20 directly controls:
// P0019R8 atomic_ref
// P0020R6 atomic<float>, atomic<double>, atomic<long double>
// P0053R7 <syncstream>
// P0122R7 <span>
// P0202R3 constexpr For <algorithm> And exchange()
// P0318R1 unwrap_reference, unwrap_ref_decay
// P0325R4 to_array()
// P0339R6 polymorphic_allocator<>
// P0355R7 <chrono> Calendars And Time Zones
// P0356R5 bind_front()
// P0357R3 Supporting Incomplete Types In reference_wrapper
// P0408R7 Efficient Access To basic_stringbuf's Buffer
// P0415R1 constexpr For <complex> (Again)
// P0439R0 enum class memory_order
// P0457R2 starts_with()/ends_with() For basic_string/basic_string_view
// P0458R2 contains() For Ordered And Unordered Associative Containers
// P0463R1 endian
// P0466R5 Layout-Compatibility And Pointer-Interconvertibility Traits
// P0475R1 Guaranteed Copy Elision For Piecewise Construction
// P0476R2 <bit> bit_cast
// P0487R1 Fixing operator>>(basic_istream&, CharT*)
// P0528R3 Atomic Compare-And-Exchange With Padding Bits
// P0550R2 remove_cvref
// P0553R4 <bit> Rotating And Counting Functions
// P0556R3 <bit> Integral Power-Of-2 Operations (renamed by P1956R1)
// P0586R2 Integer Comparison Functions
// P0591R4 Utility Functions For Uses-Allocator Construction
// P0595R2 is_constant_evaluated()
// P0616R0 Using move() In <numeric>
// P0631R8 <numbers> Math Constants
// P0645R10 <format> Text Formatting
// P0646R1 list/forward_list remove()/remove_if()/unique() Return size_type
// P0653R2 to_address()
// P0655R1 visit<R>()
// P0660R10 <stop_token> And jthread
// P0674R1 make_shared() For Arrays
// P0718R2 atomic<shared_ptr<T>>, atomic<weak_ptr<T>>
// P0753R2 osyncstream Manipulators
// P0758R1 is_nothrow_convertible
// P0768R1 Library Support For The Spaceship Comparison Operator <=>
// P0769R2 shift_left(), shift_right()
// P0784R7 Library Support For More constexpr Containers
// P0811R3 midpoint(), lerp()
// P0849R8 auto(x): decay-copy In The Language
//     (library part only)
// P0879R0 constexpr For Swapping Functions
// P0887R1 type_identity
// P0896R4 Ranges
// P0898R3 Standard Library Concepts
// P0912R5 Library Support For Coroutines
// P0919R3 Heterogeneous Lookup For Unordered Containers
// P0966R1 string::reserve() Should Not Shrink
// P0980R1 constexpr std::string
// P1001R2 execution::unseq
// P1004R2 constexpr std::vector
// P1006R1 constexpr For pointer_traits<T*>::pointer_to()
// P1007R3 assume_aligned()
// P1020R1 Smart Pointer Creation With Default Initialization
// P1023R0 constexpr For std::array Comparisons
// P1024R3 Enhancing span Usability
// P1032R1 Miscellaneous constexpr
// P1035R7 Input Range Adaptors
// P1065R2 constexpr INVOKE
//     (except the std::invoke function which is implemented unconditionally)
// P1085R2 Removing span Comparisons
// P1115R3 erase()/erase_if() Return size_type
// P1123R0 Atomic Compare-And-Exchange With Padding Bits For atomic_ref
// P1135R6 The C++20 Synchronization Library
// P1207R4 Movability Of Single-Pass Iterators
// P1208R6 <source_location>
// P1209R0 erase_if(), erase()
// P1227R2 Signed std::ssize(), Unsigned span::size()
// P1243R4 Rangify New Algorithms
// P1248R1 Fixing Relations
// P1252R2 Ranges Design Cleanup
// P1357R1 is_bounded_array, is_unbounded_array
// P1391R4 Range Constructor For string_view
// P1394R4 Range Constructor For span
// P1423R3 char8_t Backward Compatibility Remediation
// P1456R1 Move-Only Views
// P1474R1 Helpful Pointers For contiguous_iterator
// P1522R1 Iterator Difference Type And Integer Overflow
// P1523R1 Views And Size Types
// P1612R1 Relocating endian To <bit>
// P1614R2 Adding Spaceship <=> To The Library
// P1638R1 basic_istream_view::iterator Should Not Be Copyable
// P1645R1 constexpr For <numeric> Algorithms
// P1651R0 bind_front() Should Not Unwrap reference_wrapper
// P1690R1 Refining Heterogeneous Lookup For Unordered Containers
// P1716R3 Range Comparison Algorithms Are Over-Constrained
// P1739R4 Avoiding Template Bloat For Ranges
// P1754R1 Rename Concepts To standard_case
// P1862R1 Range Adaptors For Non-Copyable Iterators
// P1865R1 Adding max() To latch And barrier
// P1870R1 Rename forwarding-range To borrowed_range (Was safe_range before LWG-3379)
// P1871R1 disable_sized_sentinel_for
// P1872R0 span Should Have size_type, Not index_type
// P1878R1 Constraining Readable Types
// P1907R2 ranges::ssize
// P1956R1 <bit> has_single_bit(), bit_ceil(), bit_floor(), bit_width()
// P1959R0 Removing weak_equality And strong_equality
// P1960R0 atomic_ref Cleanup
// P1964R2 Replacing boolean With boolean-testable
// P1973R1 Renaming default_init To for_overwrite
// P1976R2 Explicit Constructors For Fixed-Extent span From Dynamic-Extent Ranges
// P1983R0 Fixing Minor Ranges Issues
// P1994R1 elements_view Needs Its Own sentinel
// P2017R1 Conditionally Borrowed Ranges
// P2091R0 Fixing Issues With Range Access CPOs
// P2102R0 Making "Implicit Expression Variations" More Explicit
// P2106R0 Range Algorithm Result Types
// P2116R0 Removing tuple-Like Protocol Support From Fixed-Extent span
// P2167R3 Improving boolean-testable Usage
// P2198R7 Freestanding Feature-Test Macros And Implementation-Defined Extensions
//     (including __cpp_lib_freestanding_ranges)
// P2210R2 Superior String Splitting
// P2216R3 std::format Improvements
// P2231R1 Completing constexpr In optional And variant
// P2251R1 Require span And basic_string_view To Be Trivially Copyable
//     (span always provides this behavior)
// P2259R1 Repairing Input Range Adaptors And counted_iterator
// P2281R1 Clarifying Range Adaptor Objects
// P2325R3 Views Should Not Be Required To Be Default Constructible
// P2328R1 join_view Should Join All views Of ranges
// P2367R0 Remove Misuses Of List-Initialization From Clause 24 Ranges
// P2372R3 Fixing Locale Handling In chrono Formatters
// P2393R1 Cleaning Up Integer-Class Types
// P2408R5 Ranges Iterators As Inputs To Non-Ranges Algorithms
// P2415R2 What Is A view?
// P2418R2 Add Support For std::generator-like Types To std::format
// P2419R2 Clarify Handling Of Encodings In Localized Formatting Of chrono Types
// P2432R1 Fix istream_view
// P2465R3 Standard Library Modules std And std.compat
// P2508R1 basic_format_string, format_string, wformat_string
// P2510R3 Formatting Pointers
// P2520R0 move_iterator<T*> Should Be A Random-Access Iterator
// P2538R1 ADL-Proof projected
// P2572R1 std::format Fill Character Allowances
// P2588R3 barrier's Phase Completion Guarantees
// P2602R2 Poison Pills Are Too Toxic
// P2609R3 Relaxing Ranges Just A Smidge
// P2655R3 common_reference_t Of reference_wrapper Should Be A Reference Type
// P2675R1 Improving std::format's Width Estimation
// P2711R1 Making Multi-Param Constructors Of Views explicit
// P2736R2 Referencing The Unicode Standard
// P2770R0 Stashing Stashing Iterators For Proper Flattening
// P2905R2 Runtime Format Strings
// P2909R4 Fix Formatting Of Code Units As Integers
// P2997R1 Removing The Common Reference Requirement From The Indirectly Invocable Concepts
// P3136R1 Retiring Niebloids
// P3323R1 Forbid atomic<cv T>, Specify atomic_ref<cv T>
//     (for atomic_ref<cv T>)

// _HAS_CXX20 indirectly controls:
// P0619R4 Removing C++17-Deprecated Features

// _HAS_CXX20 and _SILENCE_ALL_CXX20_DEPRECATION_WARNINGS control:
// P0767R1 Deprecating is_pod
// P1831R1 Deprecating volatile In The Standard Library
// Other C++20 deprecation warnings

// _HAS_CXX23 directly controls:
// P0009R18 <mdspan>
// P0288R9 move_only_function
// P0323R12 <expected>
// P0401R6 Providing Size Feedback In The Allocator Interface
// P0448R4 <spanstream>
// P0627R6 unreachable()
// P0798R8 Monadic Operations For optional
// P0881R7 <stacktrace>
// P0943R6 Supporting C Atomics In C++
// P1048R1 is_scoped_enum
// P1072R10 basic_string::resize_and_overwrite
// P1132R7 out_ptr(), inout_ptr()
// P1147R1 Printing volatile Pointers
// P1206R7 Conversions From Ranges To Containers
// P1223R5 ranges::find_last, ranges::find_last_if, ranges::find_last_if_not
// P1272R4 byteswap()
// P1328R1 constexpr type_info::operator==()
// P1425R4 Iterator Pair Constructors For stack And queue
// P1467R9 Extended Floating-Point Types
//     (only the <stdfloat> header; we don't support any optional extended floating-point types)
// P1659R3 ranges::starts_with, ranges::ends_with
// P1679R3 contains() For basic_string/basic_string_view
// P1682R3 to_underlying() For Enumerations
// P1899R3 views::stride
// P1951R1 Default Template Arguments For pair's Forwarding Constructor
// P1989R2 Range Constructor For string_view
// P2077R3 Heterogeneous Erasure Overloads For Associative Containers
// P2093R14 <print>: Formatted Output
// P2136R3 invoke_r()
// P2164R9 views::enumerate
// P2165R4 Compatibility Between tuple, pair, And tuple-like Objects
// P2166R1 Prohibiting basic_string And basic_string_view Construction From nullptr
// P2186R2 Removing Garbage Collection Support
// P2273R3 constexpr unique_ptr
// P2278R4 cbegin Should Always Return A Constant Iterator
// P2286R8 Formatting Ranges
// P2291R3 constexpr Integral <charconv>
// P2302R4 ranges::contains, ranges::contains_subrange
// P2321R2 zip
// P2322R6 ranges::fold_left, ranges::fold_right, Etc.
// P2374R4 views::cartesian_product
// P2387R3 Pipe Support For User-Defined Range Adaptors
// P2404R3 Move-Only Types For Comparison Concepts
// P2417R2 More constexpr bitset
// P2438R2 string::substr() &&
// P2440R1 ranges::iota, ranges::shift_left, ranges::shift_right
// P2441R2 views::join_with
// P2442R1 Windowing Range Adaptors: views::chunk, views::slide
// P2443R1 views::chunk_by
// P2445R1 forward_like()
// P2446R2 views::as_rvalue
// P2467R1 ios_base::noreplace: Exclusive Mode For fstreams
// P2474R2 views::repeat
// P2494R2 Relaxing Range Adaptors To Allow Move-Only Types
// P2499R0 string_view Range Constructor Should Be explicit
// P2502R2 <generator>: Synchronous Coroutine Generator For Ranges
// P2505R5 Monadic Functions For expected
// P2539R4 Synchronizing print() With The Underlying Stream
// P2540R1 Empty Product For Certain Views
// P2549R1 unexpected<E>::error()
// P2585R1 Improve Default Container Formatting
// P2599R2 mdspan: index_type, size_type
// P2604R0 mdspan: data_handle_type, data_handle(), exhaustive
// P2613R1 mdspan: empty()
// P2652R2 Disallowing User Specialization Of allocator_traits
// P2693R1 Formatting thread::id And stacktrace
// P2713R1 Escaping Improvements In std::format
// P2763R1 Fixing layout_stride's Default Constructor For Fully Static Extents
// P2787R1 pmr::generator
// P2833R2 Freestanding Library: inout expected span
//     (except for __cpp_lib_span which also covers C++26 span::at)
// P2836R1 basic_const_iterator Should Follow Its Underlying Type's Convertibility
// P3107R5 Permit An Efficient Implementation Of <print>
// P3142R0 Printing Blank Lines With println()
// P3235R3 std::print More Types Faster With Less Memory
//     (partial implementation; see GH-4924)

// _HAS_CXX23 and _SILENCE_ALL_CXX23_DEPRECATION_WARNINGS control:
// P1413R3 Deprecate aligned_storage And aligned_union
// P2614R2 Deprecating float_denorm_style, numeric_limits::has_denorm, numeric_limits::has_denorm_loss
// Other C++23 deprecation warnings

// Parallel Algorithms Notes
// C++ allows an implementation to implement parallel algorithms as calls to the serial algorithms.
// This implementation parallelizes several common algorithm calls, but not all.
//
// std::execution::unseq has no direct analogue for any optimizer we target as of 2020-07-29,
// though we will map it to #pragma loop(ivdep) for the for_each algorithms only as these are the only algorithms where
// the library does not need to introduce inter-loop-body dependencies to accomplish the algorithm's goals.
//
// The following algorithms are parallelized.
// * adjacent_difference
// * adjacent_find
// * all_of
// * any_of
// * count
// * count_if
// * destroy
// * destroy_n
// * equal
// * exclusive_scan
// * find
// * find_end
// * find_first_of
// * find_if
// * find_if_not
// * for_each
// * for_each_n
// * inclusive_scan
// * is_heap
// * is_heap_until
// * is_partitioned
// * is_sorted
// * is_sorted_until
// * mismatch
// * none_of
// * partition
// * reduce
// * remove
// * remove_if
// * replace
// * replace_if
// * search
// * search_n
// * set_difference
// * set_intersection
// * sort
// * stable_sort
// * transform
// * transform_exclusive_scan
// * transform_inclusive_scan
// * transform_reduce
// * uninitialized_default_construct
// * uninitialized_default_construct_n
// * uninitialized_value_construct
// * uninitialized_value_construct_n
//
// The following are not presently parallelized:
//
// No apparent parallelism performance improvement on target hardware; all algorithms which
// merely copy or permute elements with no branches are typically memory bandwidth limited.
// * copy
// * copy_n
// * fill
// * fill_n
// * move
// * reverse
// * reverse_copy
// * rotate
// * rotate_copy
// * shift_left
// * shift_right
// * swap_ranges
//
// Possibly same as above, but not yet tested.
// * uninitialized_copy
// * uninitialized_copy_n
// * uninitialized_fill
// * uninitialized_fill_n
// * uninitialized_move
// * uninitialized_move_n
//
// Confusion over user parallelism requirements exists; likely in the above category anyway.
// * generate
// * generate_n
//
// Effective parallelism suspected to be infeasible.
// * partial_sort
// * partial_sort_copy
//
// Not yet evaluated; parallelism may be implemented in a future release and is suspected to be beneficial.
// * copy_if
// * includes
// * inplace_merge
// * lexicographical_compare
// * max_element
// * merge
// * min_element
// * minmax_element
// * nth_element
// * partition_copy
// * remove_copy
// * remove_copy_if
// * replace_copy
// * replace_copy_if
// * set_symmetric_difference
// * set_union
// * stable_partition
// * unique
// * unique_copy

#include <vcruntime.h>
#include <xkeycheck.h> // The _HAS_CXX tags must be defined before including this.

#define _STL_STRINGIZE_(S) #S
#define _STL_STRINGIZE(S)  _STL_STRINGIZE_(S)

#define _STL_PRAGMA(PRAGMA)          _Pragma(#PRAGMA)
#define _STL_PRAGMA_MESSAGE(MESSAGE) _STL_PRAGMA(message(MESSAGE))
#define _EMIT_STL_MESSAGE(MESSAGE)   _STL_PRAGMA_MESSAGE(__FILE__ "(" _STL_STRINGIZE(__LINE__) "): " MESSAGE)

#define _EMIT_STL_WARNING(NUMBER, MESSAGE) _EMIT_STL_MESSAGE("warning " #NUMBER ": " MESSAGE)
#define _EMIT_STL_ERROR(NUMBER, MESSAGE)   static_assert(false, "error " #NUMBER ": " MESSAGE)

#ifndef _STL_WARNING_LEVEL
#if defined(_MSVC_WARNING_LEVEL) && _MSVC_WARNING_LEVEL >= 4
#define _STL_WARNING_LEVEL 4
#else // defined(_MSVC_WARNING_LEVEL) && _MSVC_WARNING_LEVEL >= 4
#define _STL_WARNING_LEVEL 3
#endif // defined(_MSVC_WARNING_LEVEL) && _MSVC_WARNING_LEVEL >= 4
#endif // !defined(_STL_WARNING_LEVEL)

#if _STL_WARNING_LEVEL < 3
#error _STL_WARNING_LEVEL cannot be less than 3.
#endif // _STL_WARNING_LEVEL < 3

#if _STL_WARNING_LEVEL > 4
#error _STL_WARNING_LEVEL cannot be greater than 4.
#endif // _STL_WARNING_LEVEL > 4

#ifndef __has_cpp_attribute
#define _FALLTHROUGH
#elif __has_cpp_attribute(fallthrough) >= 201603L
#define _FALLTHROUGH [[fallthrough]]
#else
#define _FALLTHROUGH
#endif

// _HAS_NODISCARD (in vcruntime.h) controls:
// [[nodiscard]] attributes on STL functions

// TRANSITION, This should go to vcruntime.h
#ifndef __has_cpp_attribute
#define _NODISCARD_MSG(_Msg)
#elif __has_cpp_attribute(nodiscard) >= 201907L
#define _NODISCARD_MSG(_Msg) [[nodiscard(_Msg)]]
#elif __has_cpp_attribute(nodiscard) >= 201603L
#define _NODISCARD_MSG(_Msg) [[nodiscard]]
#else
#define _NODISCARD_MSG(_Msg)
#endif

#ifndef __has_cpp_attribute
#define _NODISCARD_CTOR
#define _NODISCARD_CTOR_MSG(_Msg)
#elif __has_cpp_attribute(nodiscard) >= 201907L
#define _NODISCARD_CTOR           _NODISCARD
#define _NODISCARD_CTOR_MSG(_Msg) _NODISCARD_MSG(_Msg)
#else // ^^^ __has_cpp_attribute(nodiscard) >= 201907L / __has_cpp_attribute(nodiscard) < 201907L vvv
#define _NODISCARD_CTOR
#define _NODISCARD_CTOR_MSG(_Msg)
#endif // ^^^ defined(__has_cpp_attribute) && __has_cpp_attribute(nodiscard) < 201907L ^^^

#define _NODISCARD_REMOVE_ALG                                                                                    \
    _NODISCARD_MSG("The 'remove' and 'remove_if' algorithms return the iterator past the last element "          \
                   "that should be kept. You need to call container.erase(result, container.end()) afterwards. " \
                   "In C++20, 'std::erase' and 'std::erase_if' are simpler replacements for these two steps.")

#define _NODISCARD_UNIQUE_ALG                                                                                \
    _NODISCARD_MSG("The 'unique' algorithm returns the iterator past the last element that should be kept. " \
                   "You need to call container.erase(result, container.end()) afterwards.")

#define _NODISCARD_EMPTY_MEMBER                                                                                     \
    _NODISCARD_MSG(                                                                                                 \
        "This member function returns a bool indicating whether the collection is empty and has no other effects. " \
        "It is not useful to call this member function and discard the return value. "                              \
        "Use the 'clear()' member function if you want to erase all elements.")

#define _NODISCARD_EMPTY_ARRAY_MEMBER                                                                          \
    _NODISCARD_MSG(                                                                                            \
        "This member function returns a bool indicating whether the array is empty and has no other effects. " \
        "It is not useful to call this member function and discard the return value. "                         \
        "There's no way to clear an array as its size is fixed.")

#define _NODISCARD_EMPTY_MEMBER_NO_CLEAR                                                                            \
    _NODISCARD_MSG(                                                                                                 \
        "This member function returns a bool indicating whether the collection is empty and has no other effects. " \
        "It is not useful to call this member function and discard the return value. "                              \
        "This collection can be cleared by assigning an empty value to it.")

#define _NODISCARD_EMPTY_NON_MEMBER                                                               \
    _NODISCARD_MSG("This function returns a bool indicating whether the collection is empty and " \
                   "has no other effects. It is not useful to call this function and discard the return value.")

#define _NODISCARD_BARRIER_TOKEN \
    _NODISCARD_MSG("The token from 'arrive()' should not be discarded; it should be passed to 'wait()'.")

#define _NODISCARD_TRY_WAIT                                                                                    \
    _NODISCARD_MSG(                                                                                            \
        "This member function returns the state of the synchronization object and does not do anything else; " \
        "it is not useful to call this member function and discard the return value.")

#define _NODISCARD_TRY_CHANGE_STATE                                                                    \
    _NODISCARD_MSG("This function returns whether the operation succeeded in modifying object state. " \
                   "It is dangerous to ignore the return value.")

#define _NODISCARD_SMART_PTR_ALLOC                                                                            \
    _NODISCARD_MSG("This function constructs an object wrapped by a smart pointer and has no other effects; " \
                   "it is not useful to call this function and discard the return value.")

#define _NODISCARD_RAW_PTR_ALLOC                                                \
    _NODISCARD_MSG("This function allocates memory and returns a raw pointer. " \
                   "Discarding the return value will cause a memory leak.")

#define _NODISCARD_ASSUME_ALIGNED                                                                                    \
    _NODISCARD_MSG("'std::assume_aligned' has a potential effect on the return value (not on the passed argument). " \
                   "It is not useful to call 'std::assume_aligned' and discard the return value.")

#define _NODISCARD_LAUNDER                                                                                    \
    _NODISCARD_MSG("'std::launder' has a potential effect on the return value (not on the passed argument). " \
                   "It is not useful to call 'std::launder' and discard the return value.")

#ifdef _SILENCE_NODISCARD_LOCK_WARNINGS

#define _NODISCARD_LOCK
#define _NODISCARD_CTOR_LOCK

#else // ^^^ defined(_SILENCE_NODISCARD_LOCK_WARNINGS) / !defined(_SILENCE_NODISCARD_LOCK_WARNINGS) vvv

#define _NODISCARD_LOCK                                                                                                \
    _NODISCARD_MSG("A lock should be stored in a variable to protect the scope. If you're intentionally constructing " \
                   "a temporary to protect the rest of the current expression using the comma operator, you can cast " \
                   "the temporary to void or define _SILENCE_NODISCARD_LOCK_WARNINGS to suppress this warning.")

#define _NODISCARD_CTOR_LOCK                                                                                \
    _NODISCARD_CTOR_MSG(                                                                                    \
        "A lock should be stored in a variable to protect the scope. If you're intentionally constructing " \
        "a temporary to protect the rest of the current expression using the comma operator, you can cast " \
        "the temporary to void or define _SILENCE_NODISCARD_LOCK_WARNINGS to suppress this warning.")

#endif // ^^^ !defined(_SILENCE_NODISCARD_LOCK_WARNINGS) ^^^

#define _NODISCARD_CTOR_THREAD                                                     \
    _NODISCARD_CTOR_MSG("This temporary 'std::thread' is not joined or detached, " \
                        "so 'std::terminate' will be called at the end of the statement.")

#define _NODISCARD_CTOR_JTHREAD                                                                            \
    _NODISCARD_CTOR_MSG("This temporary 'std::jthread' is implicitly joined at the end of the statement. " \
                        "If this is intentional, you can add '.join()' to suppress this warning. "         \
                        "Otherwise, this 'std::jthread' should be stored in a variable.")

#define _NODISCARD_ASYNC                                                                                           \
    _NODISCARD_MSG("The result of 'std::async' should be stored in a variable. If the return value is discarded, " \
                   "the temporary 'std::future' is destroyed, waiting for an async result or evaluating "          \
                   "a deferred result, thus defeating the purpose of 'std::async'.")

#define _NODISCARD_GET_FUTURE \
    _NODISCARD_MSG("Since 'get_future' may be called only once, discarding the result is likely a mistake.")

#pragma push_macro("msvc")
#pragma push_macro("known_semantics")
#pragma push_macro("noop_dtor")
#pragma push_macro("intrinsic")
#pragma push_macro("lifetimebound")
#undef msvc
#undef known_semantics
#undef noop_dtor
#undef intrinsic
#undef lifetimebound

#ifndef __has_cpp_attribute
#define _HAS_MSVC_ATTRIBUTE(x) 0
#else
#define _HAS_MSVC_ATTRIBUTE(x) __has_cpp_attribute(msvc::x)
#endif

// Should we use [[msvc::known_semantics]] to tell the compiler that certain
// type trait specializations have the standard-mandated semantics?
#if _HAS_MSVC_ATTRIBUTE(known_semantics)
#define _MSVC_KNOWN_SEMANTICS [[msvc::known_semantics]]
#else
#define _MSVC_KNOWN_SEMANTICS
#endif

// Should we use [[msvc::noop_dtor]] to tell the compiler that some non-trivial
// destructors have no effects?
#if _HAS_MSVC_ATTRIBUTE(noop_dtor)
#define _MSVC_NOOP_DTOR [[msvc::noop_dtor]]
#else
#define _MSVC_NOOP_DTOR
#endif

// Should we use [[msvc::intrinsic]] allowing the compiler to implement the
// behavior of certain trivial functions?
#if _HAS_MSVC_ATTRIBUTE(intrinsic)
#define _MSVC_INTRINSIC [[msvc::intrinsic]]
#else
#define _MSVC_INTRINSIC
#endif

// Should we enable [[msvc::lifetimebound]] or [[clang::lifetimebound]] warnings?
#if !defined(__has_cpp_attribute) || defined(_SILENCE_LIFETIMEBOUND_WARNING)
#define _MSVC_LIFETIMEBOUND
#elif _HAS_MSVC_ATTRIBUTE(lifetimebound)
#define _MSVC_LIFETIMEBOUND [[msvc::lifetimebound]]
#elif __has_cpp_attribute(_Clang::__lifetimebound__)
#define _MSVC_LIFETIMEBOUND [[_Clang::__lifetimebound__]]
#else
#define _MSVC_LIFETIMEBOUND
#endif

#undef _HAS_MSVC_ATTRIBUTE
#pragma pop_macro("lifetimebound")
#pragma pop_macro("intrinsic")
#pragma pop_macro("noop_dtor")
#pragma pop_macro("known_semantics")
#pragma pop_macro("msvc")

// warning C4577: 'noexcept' used with no exception handling mode specified;
// termination on exception is not guaranteed. Specify /EHsc (/Wall)
#if _HAS_EXCEPTIONS
#define _STL_DISABLED_WARNING_C4577
#else
#define _STL_DISABLED_WARNING_C4577 4577
#endif

// warning C4984: 'if constexpr' is a C++17 language extension
#if !_HAS_CXX17
#define _STL_DISABLED_WARNING_C4984 4984
#else
#define _STL_DISABLED_WARNING_C4984
#endif

// warning C5053: support for 'explicit(<expr>)' in C++17 and earlier is a vendor extension
#if !_HAS_CXX20
#define _STL_DISABLED_WARNING_C5053 5053
#else
#define _STL_DISABLED_WARNING_C5053
#endif

#ifndef _STL_EXTRA_DISABLED_WARNINGS
#define _STL_EXTRA_DISABLED_WARNINGS
#endif // !defined(_STL_EXTRA_DISABLED_WARNINGS)

// warning C4180: qualifier applied to function type has no meaning; ignored
// warning C4324: structure was padded due to alignment specifier
// warning C4412: function signature contains type 'meow'; C++ objects are unsafe to pass between pure code
//                and mixed or native. (/Wall)
// warning C4455: literal suffix identifiers that do not start with an underscore are reserved
// warning C4494: Ignoring __declspec(allocator) because the function return type is not a pointer or reference
// warning C4514: unreferenced inline function has been removed (/Wall)
// warning C4574: 'MACRO' is defined to be '0': did you mean to use '#if MACRO'? (/Wall)
// warning C4582: 'union': constructor is not implicitly called (/Wall)
// warning C4583: 'union': destructor is not implicitly called (/Wall)
// warning C4587: behavior change: constructor is no longer implicitly called (/Wall)
// warning C4588: behavior change: destructor is no longer implicitly called (/Wall)
// warning C4619: #pragma warning: there is no warning number 'number' (/Wall)
// warning C4623: default constructor was implicitly defined as deleted (/Wall)
// warning C4625: copy constructor was implicitly defined as deleted (/Wall)
// warning C4626: assignment operator was implicitly defined as deleted (/Wall)
// warning C4643: Forward declaring 'meow' in namespace std is not permitted by the C++ Standard. (/Wall)
// warning C4648: standard attribute 'meow' is ignored
// warning C4702: unreachable code
// warning C4793: function compiled as native
// warning C4820: 'N' bytes padding added after data member 'meow' (/Wall)
// warning C4868: compiler may not enforce left-to-right evaluation order in braced initializer list (/Wall)
// warning C4988: variable declared outside class/function scope (/Wall /d1WarnOnGlobals)
// warning C5026: move constructor was implicitly defined as deleted (/Wall)
// warning C5027: move assignment operator was implicitly defined as deleted (/Wall)
// warning C5045: Compiler will insert Spectre mitigation for memory load if /Qspectre switch specified (/Wall)
// warning C5220: a non-static data member with a volatile qualified type no longer implies that compiler generated
//                copy/move constructors and copy/move assignment operators are not trivial (/Wall)
// warning C5246: 'member': the initialization of a subobject should be wrapped in braces (/Wall)
// warning C5278: adding a specialization for 'type trait' has undefined behavior
// warning C5280: a static operator '()' requires at least '/std:c++23preview'
// warning C5281: a static lambda requires at least '/std:c++23preview'
// warning C6294: Ill-defined for-loop: initial condition does not satisfy test. Loop body not executed

#ifndef _STL_DISABLED_WARNINGS
// clang-format off
#define _STL_DISABLED_WARNINGS                        \
    4180 4324 4412 4455 4494 4514 4574 4582 4583 4587 \
    4588 4619 4623 4625 4626 4643 4648 4702 4793 4820 \
    4868 4988 5026 5027 5045 5220 5246 5278 5280 5281 \
    6294                                              \
    _STL_DISABLED_WARNING_C4577                       \
    _STL_DISABLED_WARNING_C4984                       \
    _STL_DISABLED_WARNING_C5053                       \
    _STL_EXTRA_DISABLED_WARNINGS
// clang-format on
#endif // !defined(_STL_DISABLED_WARNINGS)

// warning: constexpr if is a C++17 extension [-Wc++17-extensions]
// warning: explicit(bool) is a C++20 extension [-Wc++20-extensions]
// warning: declaring overloaded 'operator()' as 'static' is a C++23 extension [-Wc++23-extensions]
// warning: static lambdas are a C++23 extension [-Wc++23-extensions]
// warning: ignoring __declspec(allocator) because the function return type '%s' is not a pointer or reference type
//     [-Wignored-attributes]
// warning: '#pragma float_control' is not supported on this target - ignored [-Wignored-pragmas]
// warning: user-defined literal suffixes not starting with '_' are reserved [-Wuser-defined-literals]
// warning: unknown pragma ignored [-Wunknown-pragmas]
#ifndef _STL_DISABLE_CLANG_WARNINGS
#ifdef __clang__
// clang-format off
#define _STL_DISABLE_CLANG_WARNINGS                                 \
    _Pragma("clang diagnostic push")                                \
    _Pragma("clang diagnostic ignored \"-Wc++17-extensions\"")      \
    _Pragma("clang diagnostic ignored \"-Wc++20-extensions\"")      \
    _Pragma("clang diagnostic ignored \"-Wc++23-extensions\"")      \
    _Pragma("clang diagnostic ignored \"-Wignored-attributes\"")    \
    _Pragma("clang diagnostic ignored \"-Wignored-pragmas\"")       \
    _Pragma("clang diagnostic ignored \"-Wuser-defined-literals\"") \
    _Pragma("clang diagnostic ignored \"-Wunknown-pragmas\"")
// clang-format on
#else // ^^^ defined(__clang__) / !defined(__clang__) vvv
#define _STL_DISABLE_CLANG_WARNINGS
#endif // ^^^ !defined(__clang__) ^^^
#endif // !defined(_STL_DISABLE_CLANG_WARNINGS)

#ifndef _STL_RESTORE_CLANG_WARNINGS
#ifdef __clang__
#define _STL_RESTORE_CLANG_WARNINGS _Pragma("clang diagnostic pop")
#else // ^^^ defined(__clang__) / !defined(__clang__) vvv
#define _STL_RESTORE_CLANG_WARNINGS
#endif // ^^^ !defined(__clang__) ^^^
#endif // !defined(_STL_RESTORE_CLANG_WARNINGS)

// warning: use of NaN is undefined behavior due to the currently enabled
//     floating-point options [-Wnan-infinity-disabled]
// warning: use of infinity is undefined behavior due to the currently enabled
//     floating-point options [-Wnan-infinity-disabled]
#ifndef _STL_DISABLE_CLANG_WARNING_NAN_INF_DISABLED
#ifdef __clang__
// clang-format off
#define _STL_DISABLE_CLANG_WARNING_NAN_INF_DISABLED \
    _Pragma("clang diagnostic push")                \
    _Pragma("clang diagnostic ignored \"-Wnan-infinity-disabled\"")
// clang-format on
#else // ^^^ defined(__clang__) / !defined(__clang__) vvv
#define _STL_DISABLE_CLANG_WARNING_NAN_INF_DISABLED
#endif // ^^^ !defined(__clang__) ^^^
#endif // !defined(_STL_DISABLE_CLANG_WARNING_NAN_INF_DISABLED)

#ifndef _STL_RESTORE_CLANG_WARNING_NAN_INF_DISABLED
#ifdef __clang__
#define _STL_RESTORE_CLANG_WARNING_NAN_INF_DISABLED _Pragma("clang diagnostic pop")
#else // ^^^ defined(__clang__) / !defined(__clang__) vvv
#define _STL_RESTORE_CLANG_WARNING_NAN_INF_DISABLED
#endif // ^^^ !defined(__clang__) ^^^
#endif // !defined(_STL_RESTORE_CLANG_WARNING_NAN_INF_DISABLED)

// clang-format off
#ifndef _STL_DISABLE_DEPRECATED_WARNING
#ifdef __clang__
#define _STL_DISABLE_DEPRECATED_WARNING \
    _Pragma("clang diagnostic push")    \
    _Pragma("clang diagnostic ignored \"-Wdeprecated-declarations\"")
#else // ^^^ defined(__clang__) / !defined(__clang__) vvv
#define _STL_DISABLE_DEPRECATED_WARNING \
    _Pragma("warning(push)")            \
    _Pragma("warning(disable : 4996)") // was declared deprecated
#endif // ^^^ !defined(__clang__) ^^^
#endif // _STL_DISABLE_DEPRECATED_WARNING
// clang-format on

#ifndef _STL_RESTORE_DEPRECATED_WARNING
#ifdef __clang__
#define _STL_RESTORE_DEPRECATED_WARNING _Pragma("clang diagnostic pop")
#else // ^^^ defined(__clang__) / !defined(__clang__) vvv
#define _STL_RESTORE_DEPRECATED_WARNING _Pragma("warning(pop)")
#endif // ^^^ !defined(__clang__) ^^^
#endif // !defined(_STL_RESTORE_DEPRECATED_WARNING)

#define _CPPLIB_VER       650
#define _MSVC_STL_VERSION 143
#define _MSVC_STL_UPDATE  202503L

#ifndef _ALLOW_COMPILER_AND_STL_VERSION_MISMATCH
#if defined(__CUDACC__) && defined(__CUDACC_VER_MAJOR__)
#if __CUDACC_VER_MAJOR__ < 12 || (__CUDACC_VER_MAJOR__ == 12 && __CUDACC_VER_MINOR__ < 4)
_EMIT_STL_ERROR(STL1002, "Unexpected compiler version, expected CUDA 12.4 or newer.");
#endif // ^^^ old CUDA ^^^
#elif defined(__EDG__)
// not attempting to detect __EDG_VERSION__ being less than expected
#elif defined(__clang__)
#if __clang_major__ < 19
_EMIT_STL_ERROR(STL1000, "Unexpected compiler version, expected Clang 19.0.0 or newer.");
#endif // ^^^ old Clang ^^^
#elif defined(_MSC_VER)
#if _MSC_VER < 1944 // Coarse-grained, not inspecting _MSC_FULL_VER
_EMIT_STL_ERROR(STL1001, "Unexpected compiler version, expected MSVC 19.44 or newer.");
#endif // ^^^ old MSVC ^^^
#else // vvv other compilers vvv
// not attempting to detect other compilers
#endif // ^^^ other compilers ^^^
#endif // !defined(_ALLOW_COMPILER_AND_STL_VERSION_MISMATCH)

#ifndef _HAS_STATIC_RTTI
#define _HAS_STATIC_RTTI 1
#endif // !defined(_HAS_STATIC_RTTI)

#if defined(_CPPRTTI) && !_HAS_STATIC_RTTI
#error /GR implies _HAS_STATIC_RTTI.
#endif // defined(_CPPRTTI) && !_HAS_STATIC_RTTI

// N4950 [dcl.constexpr]/1: "A function or static data member declared with the
// constexpr or consteval specifier is implicitly an inline function or variable"

// Functions that became constexpr in C++17
#if _HAS_CXX17
#define _CONSTEXPR17 constexpr
#else // ^^^ constexpr in C++17 and later / inline (not constexpr) in C++14 vvv
#define _CONSTEXPR17 inline
#endif // ^^^ inline (not constexpr) in C++14 ^^^

// Functions that became constexpr in C++20
#if _HAS_CXX20
#define _CONSTEXPR20 constexpr
#else // ^^^ constexpr in C++20 and later / inline (not constexpr) in C++17 and earlier vvv
#define _CONSTEXPR20 inline
#endif // ^^^ inline (not constexpr) in C++17 and earlier ^^^

// Functions that became constexpr in C++23
#if _HAS_CXX23
#define _CONSTEXPR23 constexpr
#else // ^^^ constexpr in C++23 and later / inline (not constexpr) in C++20 and earlier vvv
#define _CONSTEXPR23 inline
#endif // ^^^ inline (not constexpr) in C++20 and earlier ^^^

// P2465R3 Standard Library Modules std And std.compat
#ifdef _BUILD_STD_MODULE
#if !_HAS_CXX20
#error The Standard Library Modules are available only with C++20 or later.
#endif // ^^^ !_HAS_CXX20 ^^^
#define _EXPORT_STD export
#else // ^^^ defined(_BUILD_STD_MODULE) / !defined(_BUILD_STD_MODULE) vvv
#define _EXPORT_STD
#endif // ^^^ !defined(_BUILD_STD_MODULE) ^^^

// P0607R0 Inline Variables For The STL
#if _HAS_CXX17
#define _INLINE_VAR inline
#else
#define _INLINE_VAR
#endif

// N4190 Removing auto_ptr, random_shuffle(), And Old <functional> Stuff
#ifndef _HAS_AUTO_PTR_ETC
#define _HAS_AUTO_PTR_ETC (!_HAS_CXX17)
#endif // !defined(_HAS_AUTO_PTR_ETC)

// P0003R5 Removing Dynamic Exception Specifications
#ifndef _HAS_UNEXPECTED
#define _HAS_UNEXPECTED (!_HAS_CXX17)
#endif // !defined(_HAS_UNEXPECTED)

#if _HAS_UNEXPECTED && _HAS_CXX23
_EMIT_STL_ERROR(STL1004, "C++98 unexpected() is incompatible with C++23 unexpected<E>.");
#endif // _HAS_UNEXPECTED && _HAS_CXX23

// P0004R1 Removing Deprecated Iostreams Aliases
#ifndef _HAS_OLD_IOSTREAMS_MEMBERS
#define _HAS_OLD_IOSTREAMS_MEMBERS (!_HAS_CXX17)
#endif // !defined(_HAS_OLD_IOSTREAMS_MEMBERS)

// P0298R3 std::byte
#ifndef _HAS_STD_BYTE
#define _HAS_STD_BYTE _HAS_CXX17 // TRANSITION, OS-14273702
#endif // !defined(_HAS_STD_BYTE)

// P0302R1 Removing Allocator Support In std::function
// LWG-2385 function::assign allocator argument doesn't make sense
// LWG-2921 packaged_task and type-erased allocators
// LWG-2976 Dangling uses_allocator specialization for packaged_task
#ifndef _HAS_FUNCTION_ALLOCATOR_SUPPORT
#define _HAS_FUNCTION_ALLOCATOR_SUPPORT (!_HAS_CXX17)
#endif // !defined(_HAS_FUNCTION_ALLOCATOR_SUPPORT)

// The non-Standard std::tr1 namespace and TR1-only machinery
#ifndef _HAS_TR1_NAMESPACE
#define _HAS_TR1_NAMESPACE (!_HAS_CXX17)
#endif // !defined(_HAS_TR1_NAMESPACE)

// STL4000 is "_STATIC_CPPLIB is deprecated", currently in yvals.h
// STL4001 is "/clr:pure is deprecated", currently in yvals.h

#if _HAS_TR1_NAMESPACE
#ifdef _SILENCE_TR1_NAMESPACE_DEPRECATION_WARNING
#define _DEPRECATE_TR1_NAMESPACE
#else // ^^^ warning disabled / warning enabled vvv
#define _DEPRECATE_TR1_NAMESPACE                                                                                  \
    [[deprecated(                                                                                                 \
        "warning STL4002: "                                                                                       \
        "The non-Standard std::tr1 namespace and TR1-only machinery are deprecated and will be REMOVED. You can " \
        "define _SILENCE_TR1_NAMESPACE_DEPRECATION_WARNING to suppress this warning.")]]
#endif // ^^^ warning enabled ^^^
#endif // _HAS_TR1_NAMESPACE

// STL4003 was "The non-Standard std::identity struct is deprecated and will be REMOVED."

// Enforcement of matching allocator value_types
#ifndef _ENFORCE_MATCHING_ALLOCATORS
#define _ENFORCE_MATCHING_ALLOCATORS _HAS_CXX17
#endif // !defined(_ENFORCE_MATCHING_ALLOCATORS)

#define _MISMATCHED_ALLOCATOR_MESSAGE(_CONTAINER, _VALUE_TYPE)                                                      \
    _CONTAINER " requires that Allocator's value_type match " _VALUE_TYPE " (See N4950 [container.alloc.reqmts]/5)" \
               " Either fix the allocator value_type or define _ENFORCE_MATCHING_ALLOCATORS=0"                      \
               " to suppress this error."

// Enforcement of Standard facet specializations
#ifndef _ENFORCE_FACET_SPECIALIZATIONS
#define _ENFORCE_FACET_SPECIALIZATIONS 0
#endif // !defined(_ENFORCE_FACET_SPECIALIZATIONS)

#define _FACET_SPECIALIZATION_MESSAGE                                                  \
    "Unsupported facet specialization; see N4950 [locale.category]. "                  \
    "Either use a Standard specialization or define _ENFORCE_FACET_SPECIALIZATIONS=0 " \
    "to suppress this error."

// To improve compiler throughput, use 'hidden friend' operators in <system_error> instead of non-members that are
// depicted in the Standard.
#ifndef _STL_OPTIMIZE_SYSTEM_ERROR_OPERATORS
#define _STL_OPTIMIZE_SYSTEM_ERROR_OPERATORS 1
#endif // !defined(_STL_OPTIMIZE_SYSTEM_ERROR_OPERATORS)

// Controls whether the STL will force /fp:fast to enable vectorization of algorithms defined
// in the standard as special cases; such as reduce, transform_reduce, inclusive_scan, exclusive_scan
#ifndef _STD_VECTORIZE_WITH_FLOAT_CONTROL
#ifdef _M_FP_EXCEPT
#define _STD_VECTORIZE_WITH_FLOAT_CONTROL 0
#else // ^^^ floating-point exceptions enabled / floating-point exceptions disabled (default) vvv
#define _STD_VECTORIZE_WITH_FLOAT_CONTROL 1
#endif // ^^^ floating-point exceptions disabled (default) ^^^
#endif // !defined(_STD_VECTORIZE_WITH_FLOAT_CONTROL)

// P0174R2 Deprecating Vestigial Library Parts
// P0521R0 Deprecating shared_ptr::unique()
// Other C++17 deprecation warnings

// N4659 D.4 [depr.cpp.headers]
#if _HAS_CXX17 && !defined(_SILENCE_CXX17_C_HEADER_DEPRECATION_WARNING) \
    && !defined(_SILENCE_ALL_CXX17_DEPRECATION_WARNINGS)
#define _CXX17_DEPRECATE_C_HEADER                                                               \
    [[deprecated("warning STL4004: "                                                            \
                 "<ccomplex>, <cstdalign>, <cstdbool>, and <ctgmath> are deprecated in C++17. " \
                 "You can define _SILENCE_CXX17_C_HEADER_DEPRECATION_WARNING "                  \
                 "or _SILENCE_ALL_CXX17_DEPRECATION_WARNINGS to suppress this warning.")]]
#else // ^^^ warning enabled / warning disabled vvv
#define _CXX17_DEPRECATE_C_HEADER
#endif // ^^^ warning disabled ^^^

// N4659 D.6 [depr.str.strstreams]
#if _HAS_CXX17 && !defined(_SILENCE_CXX17_STRSTREAM_DEPRECATION_WARNING) \
    && !defined(_SILENCE_ALL_CXX17_DEPRECATION_WARNINGS)
#define _CXX17_DEPRECATE_STRSTREAM                                              \
    [[deprecated("warning STL4005: <strstream> is deprecated in C++17. "        \
                 "You can define _SILENCE_CXX17_STRSTREAM_DEPRECATION_WARNING " \
                 "or _SILENCE_ALL_CXX17_DEPRECATION_WARNINGS to suppress this warning.")]]
#else // ^^^ warning enabled / warning disabled vvv
#define _CXX17_DEPRECATE_STRSTREAM
#endif // ^^^ warning disabled ^^^

// N4659 D.7 [depr.uncaught]
#if _HAS_CXX17 && !defined(_SILENCE_CXX17_UNCAUGHT_EXCEPTION_DEPRECATION_WARNING) \
    && !defined(_SILENCE_ALL_CXX17_DEPRECATION_WARNINGS)
#define _CXX17_DEPRECATE_UNCAUGHT_EXCEPTION                                              \
    [[deprecated("warning STL4006: "                                                     \
                 "std::uncaught_exception() is deprecated in C++17. "                    \
                 "It is superseded by std::uncaught_exceptions(), plural. "              \
                 "You can define _SILENCE_CXX17_UNCAUGHT_EXCEPTION_DEPRECATION_WARNING " \
                 "or _SILENCE_ALL_CXX17_DEPRECATION_WARNINGS to suppress this warning.")]]
#else // ^^^ warning enabled / warning disabled vvv
#define _CXX17_DEPRECATE_UNCAUGHT_EXCEPTION
#endif // ^^^ warning disabled ^^^

// N4659 D.8.1 [depr.weak.result_type]
// N4659 D.8.2 [depr.func.adaptor.typedefs]
#if _HAS_CXX17 && !defined(_SILENCE_CXX17_ADAPTOR_TYPEDEFS_DEPRECATION_WARNING) \
    && !defined(_SILENCE_ALL_CXX17_DEPRECATION_WARNINGS)
#define _CXX17_DEPRECATE_ADAPTOR_TYPEDEFS                                                                         \
    [[deprecated(                                                                                                 \
        "warning STL4007: Many result_type typedefs "                                                             \
        "and all argument_type, first_argument_type, and second_argument_type typedefs are deprecated in C++17. " \
        "You can define _SILENCE_CXX17_ADAPTOR_TYPEDEFS_DEPRECATION_WARNING "                                     \
        "or _SILENCE_ALL_CXX17_DEPRECATION_WARNINGS to suppress this warning.")]]
#else // ^^^ warning enabled / warning disabled vvv
#define _CXX17_DEPRECATE_ADAPTOR_TYPEDEFS
#endif // ^^^ warning disabled ^^^

// N4659 D.8.3 [depr.negators]
#if _HAS_CXX17 && !defined(_SILENCE_CXX17_NEGATORS_DEPRECATION_WARNING) \
    && !defined(_SILENCE_ALL_CXX17_DEPRECATION_WARNINGS)
#define _CXX17_DEPRECATE_NEGATORS                                                                                \
    [[deprecated("warning STL4008: "                                                                             \
                 "std::not1(), std::not2(), std::unary_negate, and std::binary_negate are deprecated in C++17. " \
                 "They are superseded by std::not_fn(). "                                                        \
                 "You can define _SILENCE_CXX17_NEGATORS_DEPRECATION_WARNING "                                   \
                 "or _SILENCE_ALL_CXX17_DEPRECATION_WARNINGS to suppress this warning.")]]
#else // ^^^ warning enabled / warning disabled vvv
#define _CXX17_DEPRECATE_NEGATORS
#endif // ^^^ warning disabled ^^^

// STL4009 was "std::allocator<void> is deprecated in C++17"

// N4659 D.9 [depr.default.allocator]
#if _HAS_CXX17 && !defined(_SILENCE_CXX17_OLD_ALLOCATOR_MEMBERS_DEPRECATION_WARNING) \
    && !defined(_SILENCE_ALL_CXX17_DEPRECATION_WARNINGS)
#define _CXX17_DEPRECATE_OLD_ALLOCATOR_MEMBERS                                              \
    [[deprecated("warning STL4010: "                                                        \
                 "Various members of std::allocator are deprecated in C++17. "              \
                 "Use std::allocator_traits instead of accessing these members directly. "  \
                 "You can define _SILENCE_CXX17_OLD_ALLOCATOR_MEMBERS_DEPRECATION_WARNING " \
                 "or _SILENCE_ALL_CXX17_DEPRECATION_WARNINGS to suppress this warning.")]]
#else // ^^^ warning enabled / warning disabled vvv
#define _CXX17_DEPRECATE_OLD_ALLOCATOR_MEMBERS
#endif // ^^^ warning disabled ^^^

// N4659 D.10 [depr.storage.iterator]
#if _HAS_CXX17 && !defined(_SILENCE_CXX17_RAW_STORAGE_ITERATOR_DEPRECATION_WARNING) \
    && !defined(_SILENCE_ALL_CXX17_DEPRECATION_WARNINGS)
#define _CXX17_DEPRECATE_RAW_STORAGE_ITERATOR                                                  \
    [[deprecated("warning STL4011: "                                                           \
                 "std::raw_storage_iterator is deprecated in C++17. "                          \
                 "Consider using the std::uninitialized_copy() family of algorithms instead. " \
                 "You can define _SILENCE_CXX17_RAW_STORAGE_ITERATOR_DEPRECATION_WARNING "     \
                 "or _SILENCE_ALL_CXX17_DEPRECATION_WARNINGS to suppress this warning.")]]
#else // ^^^ warning enabled / warning disabled vvv
#define _CXX17_DEPRECATE_RAW_STORAGE_ITERATOR
#endif // ^^^ warning disabled ^^^

// N4659 D.11 [depr.temporary.buffer]
#if _HAS_CXX17 && !defined(_SILENCE_CXX17_TEMPORARY_BUFFER_DEPRECATION_WARNING) \
    && !defined(_SILENCE_ALL_CXX17_DEPRECATION_WARNINGS)
#define _CXX17_DEPRECATE_TEMPORARY_BUFFER                                                                   \
    [[deprecated("warning STL4012: "                                                                        \
                 "std::get_temporary_buffer() and std::return_temporary_buffer() are deprecated in C++17. " \
                 "You can define _SILENCE_CXX17_TEMPORARY_BUFFER_DEPRECATION_WARNING "                      \
                 "or _SILENCE_ALL_CXX17_DEPRECATION_WARNINGS to suppress this warning.")]]
#else // ^^^ warning enabled / warning disabled vvv
#define _CXX17_DEPRECATE_TEMPORARY_BUFFER
#endif // ^^^ warning disabled ^^^

// N4659 D.12 [depr.meta.types]
#if _HAS_CXX17 && !defined(_SILENCE_CXX17_IS_LITERAL_TYPE_DEPRECATION_WARNING) \
    && !defined(_SILENCE_ALL_CXX17_DEPRECATION_WARNINGS)
#define _CXX17_DEPRECATE_IS_LITERAL_TYPE                                                     \
    [[deprecated("warning STL4013: "                                                         \
                 "std::is_literal_type and std::is_literal_type_v are deprecated in C++17. " \
                 "You can define _SILENCE_CXX17_IS_LITERAL_TYPE_DEPRECATION_WARNING "        \
                 "or _SILENCE_ALL_CXX17_DEPRECATION_WARNINGS to suppress this warning.")]]
#else // ^^^ warning enabled / warning disabled vvv
#define _CXX17_DEPRECATE_IS_LITERAL_TYPE
#endif // ^^^ warning disabled ^^^

// N4659 D.12 [depr.meta.types]
#if _HAS_CXX17 && !defined(_SILENCE_CXX17_RESULT_OF_DEPRECATION_WARNING) \
    && !defined(_SILENCE_ALL_CXX17_DEPRECATION_WARNINGS)
#define _CXX17_DEPRECATE_RESULT_OF                                                      \
    [[deprecated("warning STL4014: "                                                    \
                 "std::result_of and std::result_of_t are deprecated in C++17. "        \
                 "They are superseded by std::invoke_result and std::invoke_result_t. " \
                 "You can define _SILENCE_CXX17_RESULT_OF_DEPRECATION_WARNING "         \
                 "or _SILENCE_ALL_CXX17_DEPRECATION_WARNINGS to suppress this warning.")]]
#else // ^^^ warning enabled / warning disabled vvv
#define _CXX17_DEPRECATE_RESULT_OF
#endif // ^^^ warning disabled ^^^

// N4659 D.13 [depr.iterator.primitives]
#if _HAS_CXX17 && !defined(_SILENCE_CXX17_ITERATOR_BASE_CLASS_DEPRECATION_WARNING) \
    && !defined(_SILENCE_ALL_CXX17_DEPRECATION_WARNINGS)
#define _CXX17_DEPRECATE_ITERATOR_BASE_CLASS                                                                          \
    [[deprecated(                                                                                                     \
        "warning STL4015: "                                                                                           \
        "The std::iterator class template (used as a base class to provide typedefs) is deprecated in C++17. "        \
        "(The <iterator> header is NOT deprecated.) The C++ Standard has never required user-defined iterators to "   \
        "derive from std::iterator. To fix this warning, stop deriving from std::iterator and start providing "       \
        "publicly accessible typedefs named iterator_category, value_type, difference_type, pointer, and reference. " \
        "Note that value_type is required to be non-const, even for constant iterators. "                             \
        "You can define _SILENCE_CXX17_ITERATOR_BASE_CLASS_DEPRECATION_WARNING "                                      \
        "or _SILENCE_ALL_CXX17_DEPRECATION_WARNINGS to suppress this warning.")]]
#else // ^^^ warning enabled / warning disabled vvv
#define _CXX17_DEPRECATE_ITERATOR_BASE_CLASS
#endif // ^^^ warning disabled ^^^

// N4659 D.14 [depr.util.smartptr.shared.obs]
#if _HAS_CXX17 && !defined(_SILENCE_CXX17_SHARED_PTR_UNIQUE_DEPRECATION_WARNING) \
    && !defined(_SILENCE_ALL_CXX17_DEPRECATION_WARNINGS)
#define _CXX17_DEPRECATE_SHARED_PTR_UNIQUE                                              \
    [[deprecated("warning STL4016: "                                                    \
                 "std::shared_ptr::unique() is deprecated in C++17. "                   \
                 "You can define _SILENCE_CXX17_SHARED_PTR_UNIQUE_DEPRECATION_WARNING " \
                 "or _SILENCE_ALL_CXX17_DEPRECATION_WARNINGS to suppress this warning.")]]
#else // ^^^ warning enabled / warning disabled vvv
#define _CXX17_DEPRECATE_SHARED_PTR_UNIQUE
#endif // ^^^ warning disabled ^^^

// N4659 D.15 [depr.locale.stdcvt]
// N4659 D.16 [depr.conversions]
#if _HAS_CXX17 && !defined(_SILENCE_CXX17_CODECVT_HEADER_DEPRECATION_WARNING) \
    && !defined(_SILENCE_ALL_CXX17_DEPRECATION_WARNINGS)
#define _CXX17_DEPRECATE_CODECVT_HEADER                                                                        \
    [[deprecated(                                                                                              \
        "warning STL4017: "                                                                                    \
        "std::wbuffer_convert, std::wstring_convert, and the <codecvt> header (containing std::codecvt_mode, " \
        "std::codecvt_utf8, std::codecvt_utf16, and std::codecvt_utf8_utf16) are deprecated in C++17. "        \
        "(The std::codecvt class template is NOT deprecated.) "                                                \
        "The C++ Standard doesn't provide equivalent non-deprecated functionality; "                           \
        "consider using MultiByteToWideChar() and WideCharToMultiByte() from <Windows.h> instead. "            \
        "You can define _SILENCE_CXX17_CODECVT_HEADER_DEPRECATION_WARNING "                                    \
        "or _SILENCE_ALL_CXX17_DEPRECATION_WARNINGS to suppress this warning.")]]
#else // ^^^ warning enabled / warning disabled vvv
#define _CXX17_DEPRECATE_CODECVT_HEADER
#endif // ^^^ warning disabled ^^^

// STL4018 was "The non-Standard std::tr2::sys namespace is deprecated and will be REMOVED."

// STL4019 was "The member std::fpos::seekpos() is non-Standard, and [...] will be removed"

// P0482R6 Library Support For char8_t
// Other C++20 deprecation warnings

// N4868 D.23 [depr.locale.category]
#if _HAS_CXX20 && !defined(_SILENCE_CXX20_CODECVT_FACETS_DEPRECATION_WARNING) \
    && !defined(_SILENCE_ALL_CXX20_DEPRECATION_WARNINGS)
#define _CXX20_DEPRECATE_CODECVT_FACETS                                                                                \
    [[deprecated("warning STL4020: "                                                                                   \
                 "std::codecvt<char16_t, char, mbstate_t>, std::codecvt<char32_t, char, mbstate_t>, "                  \
                 "std::codecvt_byname<char16_t, char, mbstate_t>, and std::codecvt_byname<char32_t, char, mbstate_t> " \
                 "are deprecated in C++20. You can define _SILENCE_CXX20_CODECVT_FACETS_DEPRECATION_WARNING "          \
                 "or _SILENCE_ALL_CXX20_DEPRECATION_WARNINGS to suppress this warning.")]]
#else // ^^^ warning enabled / warning disabled vvv
#define _CXX20_DEPRECATE_CODECVT_FACETS
#endif // ^^^ warning disabled ^^^

// N4868 D.24 [depr.fs.path.factory]
#if _HAS_CXX20 && defined(__cpp_char8_t) && !defined(_SILENCE_CXX20_U8PATH_DEPRECATION_WARNING) \
    && !defined(_SILENCE_ALL_CXX20_DEPRECATION_WARNINGS)
#define _CXX20_DEPRECATE_U8PATH                                                                                      \
    [[deprecated("warning STL4021: "                                                                                 \
                 "The std::filesystem::u8path() overloads are deprecated in C++20. "                                 \
                 "The constructors of std::filesystem::path provide equivalent functionality via construction from " \
                 "u8string, u8string_view, or iterators with value_type char8_t. "                                   \
                 "You can define _SILENCE_CXX20_U8PATH_DEPRECATION_WARNING "                                         \
                 "or _SILENCE_ALL_CXX20_DEPRECATION_WARNINGS to suppress this warning.")]]
#else // ^^^ warning enabled / warning disabled vvv
#define _CXX20_DEPRECATE_U8PATH
#endif // ^^^ warning disabled ^^^

// STL4022 warned about "The hash_meow and unordered_meow containers' non-Standard lower_bound() member"
// STL4023 warned about "The hash_meow and unordered_meow containers' non-Standard upper_bound() member"

// P0966R1 [depr.string.capacity]
#if _HAS_CXX20 && !defined(_SILENCE_CXX20_STRING_RESERVE_WITHOUT_ARGUMENT_DEPRECATION_WARNING) \
    && !defined(_SILENCE_ALL_CXX20_DEPRECATION_WARNINGS)
#define _CXX20_DEPRECATE_STRING_RESERVE_WITHOUT_ARGUMENT                                                             \
    [[deprecated("warning STL4024: "                                                                                 \
                 "std::string::reserve() without an argument is deprecated in C++20. "                               \
                 "To shrink the string's capacity, use std::string::shrink_to_fit() instead. Otherwise, provide an " \
                 "argument to std::string::reserve(). "                                                              \
                 "You can define _SILENCE_CXX20_STRING_RESERVE_WITHOUT_ARGUMENT_DEPRECATION_WARNING "                \
                 "or _SILENCE_ALL_CXX20_DEPRECATION_WARNINGS to suppress this warning.")]]
#else // ^^^ warning enabled / warning disabled vvv
#define _CXX20_DEPRECATE_STRING_RESERVE_WITHOUT_ARGUMENT
#endif // ^^^ warning disabled ^^^

// P0767R1 [depr.meta.types]
#if _HAS_CXX20 && !defined(_SILENCE_CXX20_IS_POD_DEPRECATION_WARNING) \
    && !defined(_SILENCE_ALL_CXX20_DEPRECATION_WARNINGS)
#define _CXX20_DEPRECATE_IS_POD                                                                                     \
    [[deprecated("warning STL4025: "                                                                                \
                 "std::is_pod and std::is_pod_v are deprecated in C++20. "                                          \
                 "The std::is_trivially_copyable and/or std::is_standard_layout traits likely suit your use case. " \
                 "You can define _SILENCE_CXX20_IS_POD_DEPRECATION_WARNING "                                        \
                 "or _SILENCE_ALL_CXX20_DEPRECATION_WARNINGS to suppress this warning.")]]
#else // ^^^ warning enabled / warning disabled vvv
#define _CXX20_DEPRECATE_IS_POD
#endif // ^^^ warning disabled ^^^

// STL4026 was
// "std::experimental::erase() and std::experimental::erase_if() are deprecated by Microsoft and will be REMOVED."

// P0768R1 [depr.relops]
#if _HAS_CXX20 && !defined(_SILENCE_CXX20_REL_OPS_DEPRECATION_WARNING) \
    && !defined(_SILENCE_ALL_CXX20_DEPRECATION_WARNINGS)
#define _CXX20_DEPRECATE_REL_OPS                                                                                      \
    [[deprecated("warning STL4027: "                                                                                  \
                 "The namespace std::rel_ops and its contents are deprecated in C++20. "                              \
                 "Their use is superseded by C++20's <=> operator and automatic rewrites of relational expressions. " \
                 "You can define _SILENCE_CXX20_REL_OPS_DEPRECATION_WARNING or "                                      \
                 "_SILENCE_ALL_CXX20_DEPRECATION_WARNINGS to suppress this warning.")]]
#else // ^^^ warning enabled / warning disabled vvv
#define _CXX20_DEPRECATE_REL_OPS
#endif // ^^^ warning disabled ^^^

#if _HAS_CXX20 && !defined(_SILENCE_CXX20_ATOMIC_INIT_DEPRECATION_WARNING) \
    && !defined(_SILENCE_ALL_CXX20_DEPRECATION_WARNINGS)
#define _CXX20_DEPRECATE_ATOMIC_INIT                                                  \
    [[deprecated("warning STL4028: "                                                  \
                 "std::atomic_init() overloads are deprecated in C++20. "             \
                 "The constructors of std::atomic provide equivalent functionality. " \
                 "You can define _SILENCE_CXX20_ATOMIC_INIT_DEPRECATION_WARNING "     \
                 "or _SILENCE_ALL_CXX20_DEPRECATION_WARNINGS to suppress this warning.")]]
#else // ^^^ warning enabled / warning disabled vvv
#define _CXX20_DEPRECATE_ATOMIC_INIT
#endif // ^^^ warning disabled ^^^

#if _HAS_CXX20 && !defined(_SILENCE_CXX20_OLD_SHARED_PTR_ATOMIC_SUPPORT_DEPRECATION_WARNING) \
    && !defined(_SILENCE_ALL_CXX20_DEPRECATION_WARNINGS)
#define _CXX20_DEPRECATE_OLD_SHARED_PTR_ATOMIC_SUPPORT                                              \
    [[deprecated("warning STL4029: "                                                                \
                 "std::atomic_*() overloads for shared_ptr are deprecated in C++20. "               \
                 "The shared_ptr specialization of std::atomic provides superior functionality. "   \
                 "You can define _SILENCE_CXX20_OLD_SHARED_PTR_ATOMIC_SUPPORT_DEPRECATION_WARNING " \
                 "or _SILENCE_ALL_CXX20_DEPRECATION_WARNINGS to suppress this warning.")]]
#else // ^^^ warning enabled / warning disabled vvv
#define _CXX20_DEPRECATE_OLD_SHARED_PTR_ATOMIC_SUPPORT
#endif // ^^^ warning disabled ^^^

#if _HAS_CXX20 && !defined(_SILENCE_CXX20_VOLATILE_DEPRECATION_WARNING) \
    && !defined(_SILENCE_ALL_CXX20_DEPRECATION_WARNINGS)
#define _CXX20_DEPRECATE_VOLATILE                                                                   \
    [[deprecated("warning STL4030: "                                                                \
                 "Some operations on volatile-qualified types in the STL are deprecated in C++20. " \
                 "You can define _SILENCE_CXX20_VOLATILE_DEPRECATION_WARNING "                      \
                 "or _SILENCE_ALL_CXX20_DEPRECATION_WARNINGS to suppress this warning.")]]
#else // ^^^ warning enabled / warning disabled vvv
#define _CXX20_DEPRECATE_VOLATILE
#endif // ^^^ warning disabled ^^^

#if _HAS_CXX20 && !defined(_SILENCE_CXX20_MOVE_ITERATOR_ARROW_DEPRECATION_WARNING) \
    && !defined(_SILENCE_ALL_CXX20_DEPRECATION_WARNINGS)
#define _CXX20_DEPRECATE_MOVE_ITERATOR_ARROW                                              \
    [[deprecated("warning STL4031: "                                                      \
                 "std::move_iterator::operator->() is deprecated in C++20. "              \
                 "You can define _SILENCE_CXX20_MOVE_ITERATOR_ARROW_DEPRECATION_WARNING " \
                 "or _SILENCE_ALL_CXX20_DEPRECATION_WARNINGS to suppress this warning.")]]
#else // ^^^ warning enabled / warning disabled vvv
#define _CXX20_DEPRECATE_MOVE_ITERATOR_ARROW
#endif // ^^^ warning disabled ^^^

// STL4032 was "std::pmr::polymorphic_allocator::destroy() is deprecated in C++17 by LWG-3036." (reverted by P2875R4)

#if _HAS_CXX20 && !defined(_SILENCE_CXX20_IS_ALWAYS_EQUAL_DEPRECATION_WARNING) \
    && !defined(_SILENCE_ALL_CXX20_DEPRECATION_WARNINGS)
#define _CXX20_DEPRECATE_IS_ALWAYS_EQUAL                                                \
    [[deprecated("warning STL4033: "                                                    \
                 "std::allocator::is_always_equal is deprecated in C++20 by LWG-3170. " \
                 "Prefer std::allocator_traits<allocator<T>>::is_always_equal. "        \
                 "You can define _SILENCE_CXX20_IS_ALWAYS_EQUAL_DEPRECATION_WARNING "   \
                 "or _SILENCE_ALL_CXX20_DEPRECATION_WARNINGS to suppress this warning.")]]
#else // ^^^ warning enabled / warning disabled vvv
#define _CXX20_DEPRECATE_IS_ALWAYS_EQUAL
#endif // ^^^ warning disabled ^^^

#if _HAS_CXX23 && !defined(_SILENCE_CXX23_ALIGNED_STORAGE_DEPRECATION_WARNING) \
    && !defined(_SILENCE_ALL_CXX23_DEPRECATION_WARNINGS)
#define _CXX23_DEPRECATE_ALIGNED_STORAGE                                                     \
    [[deprecated("warning STL4034: "                                                         \
                 "std::aligned_storage and std::aligned_storage_t are deprecated in C++23. " \
                 "Prefer alignas(T) std::byte t_buff[sizeof(T)]. "                           \
                 "You can define _SILENCE_CXX23_ALIGNED_STORAGE_DEPRECATION_WARNING "        \
                 "or _SILENCE_ALL_CXX23_DEPRECATION_WARNINGS to suppress this warning.")]]
#else // ^^^ warning enabled / warning disabled vvv
#define _CXX23_DEPRECATE_ALIGNED_STORAGE
#endif // ^^^ warning disabled ^^^

#if _HAS_CXX23 && !defined(_SILENCE_CXX23_ALIGNED_UNION_DEPRECATION_WARNING) \
    && !defined(_SILENCE_ALL_CXX23_DEPRECATION_WARNINGS)
#define _CXX23_DEPRECATE_ALIGNED_UNION                                                   \
    [[deprecated("warning STL4035: "                                                     \
                 "std::aligned_union and std::aligned_union_t are deprecated in C++23. " \
                 "Prefer alignas(Ts...) std::byte t_buff[std::max({sizeof(Ts)...})]. "   \
                 "You can define _SILENCE_CXX23_ALIGNED_UNION_DEPRECATION_WARNING "      \
                 "or _SILENCE_ALL_CXX23_DEPRECATION_WARNINGS to suppress this warning.")]]
#else // ^^^ warning enabled / warning disabled vvv
#define _CXX23_DEPRECATE_ALIGNED_UNION
#endif // ^^^ warning disabled ^^^

#if _HAS_CXX20 && !defined(_SILENCE_CXX20_CISO646_REMOVED_WARNING) && !defined(_SILENCE_ALL_CXX20_DEPRECATION_WARNINGS)
#define _CXX20_REMOVE_CISO646                                             \
    [[deprecated("warning STL4036: "                                      \
                 "<ciso646> is removed in C++20. "                        \
                 "You can define _SILENCE_CXX20_CISO646_REMOVED_WARNING " \
                 "or _SILENCE_ALL_CXX20_DEPRECATION_WARNINGS to suppress this warning.")]]
#else // ^^^ warning enabled / warning disabled vvv
#define _CXX20_REMOVE_CISO646
#endif // ^^^ warning disabled ^^^

#if !defined(_SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING)
#define _DEPRECATE_NONFLOATING_COMPLEX                                                 \
    [[deprecated("warning STL4037: "                                                   \
                 "The effect of instantiating the template std::complex for any "      \
                 "type other than float, double, or long double is unspecified. "      \
                 "You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to " \
                 "suppress this warning.")]]
#else // ^^^ warning enabled / warning disabled vvv
#define _DEPRECATE_NONFLOATING_COMPLEX
#endif // ^^^ warning disabled ^^^

// STL4038 is used to warn that "The contents of <meow> are available only with C++NN or later."

// STL4039 is used to warn that "The contents of <coroutine> are not available with /await."

// STL4040 is used to warn that "The contents of <any> require static RTTI."

#if _HAS_CXX23 && !defined(_SILENCE_CXX23_UNIX_STREAMS_DEPRECATION_WARNING) \
    && !defined(_SILENCE_ALL_CXX23_DEPRECATION_WARNINGS)
#define _CXX23_DEPRECATE_UNIX_STREAMS                                                                                  \
    [[deprecated(                                                                                                      \
        "warning STL4041: "                                                                                            \
        "std::errc enumerators std::errc::no_message_available, std::errc::no_stream_resources, "                      \
        "std::errc::not_a_stream, and std::errc::stream_timeout and their corresponding errno macros ENODATA, ENOSR, " \
        "ENOSTR, and ETIME are deprecated in C++23 by LWG-3869. These errno macros are deprecated in POSIX 2008 and "  \
        "removed in POSIX 202x. You can define _SILENCE_CXX23_UNIX_STREAMS_DEPRECATION_WARNING or "                    \
        "_SILENCE_ALL_CXX23_DEPRECATION_WARNINGS to suppress this warning.")]]
#else // ^^^ warning enabled / warning disabled vvv
#define _CXX23_DEPRECATE_UNIX_STREAMS
#endif // ^^^ warning disabled ^^^

#if _HAS_CXX23 && !defined(_SILENCE_CXX23_DENORM_DEPRECATION_WARNING) \
    && !defined(_SILENCE_ALL_CXX23_DEPRECATION_WARNINGS)
#define _CXX23_DEPRECATE_DENORM                                                                                        \
    [[deprecated("warning STL4042: "                                                                                   \
                 "std::float_denorm_style, std::numeric_limits::has_denorm, and std::numeric_limits::has_denorm_loss " \
                 "are deprecated in C++23. You can define _SILENCE_CXX23_DENORM_DEPRECATION_WARNING or "               \
                 "_SILENCE_ALL_CXX23_DEPRECATION_WARNINGS to suppress this warning.")]]
#else // ^^^ warning enabled / warning disabled vvv
#define _CXX23_DEPRECATE_DENORM
#endif // ^^^ warning disabled ^^^

#if !defined(_SILENCE_STDEXT_ARR_ITERS_DEPRECATION_WARNING) && !defined(_SILENCE_ALL_MS_EXT_DEPRECATION_WARNINGS)
#define _DEPRECATE_STDEXT_ARR_ITERS                                                                               \
    [[deprecated(                                                                                                 \
        "warning STL4043: stdext::checked_array_iterator, stdext::unchecked_array_iterator, and related factory " \
        "functions are non-Standard extensions and will be removed in the future. std::span (since C++20) and "   \
        "gsl::span can be used instead. You can define _SILENCE_STDEXT_ARR_ITERS_DEPRECATION_WARNING or "         \
        "_SILENCE_ALL_MS_EXT_DEPRECATION_WARNINGS to suppress this warning.")]]
#else // ^^^ warning enabled / warning disabled vvv
#define _DEPRECATE_STDEXT_ARR_ITERS
#endif // ^^^ warning disabled ^^^

// STL4044 was "The contents of the stdext::cvt namespace are non-Standard extensions and will be removed"

#if !defined(_SILENCE_IO_PFX_SFX_DEPRECATION_WARNING) && !defined(_SILENCE_ALL_MS_EXT_DEPRECATION_WARNINGS)
#define _DEPRECATE_IO_PFX_SFX                                                                                          \
    [[deprecated(                                                                                                      \
        "warning STL4045: The ipfx(), isfx(), opfx(), and osfx() functions are removed before C++98 (see WG21-N0794) " \
        "but kept as non-Standard extensions. They will be removed in the future, and the member classes sentry "      \
        "should be used instead. You can define _SILENCE_IO_PFX_SFX_DEPRECATION_WARNING or "                           \
        "_SILENCE_ALL_MS_EXT_DEPRECATION_WARNINGS to suppress this warning.")]]
#else // ^^^ warning enabled / warning disabled vvv
#define _DEPRECATE_IO_PFX_SFX
#endif // ^^^ warning disabled ^^^

#if !defined(_SILENCE_TR1_NAMESPACE_DEPRECATION_WARNING) && !defined(_SILENCE_TR1_RANDOM_DEPRECATION_WARNING) \
    && !defined(_SILENCE_ALL_MS_EXT_DEPRECATION_WARNINGS)
#define _DEPRECATE_TR1_RANDOM                                                                                          \
    [[deprecated("warning STL4046: Non-Standard TR1 components in <random> are deprecated and will be REMOVED. You "   \
                 "can define _SILENCE_TR1_NAMESPACE_DEPRECATION_WARNING, _SILENCE_TR1_RANDOM_DEPRECATION_WARNING, or " \
                 "_SILENCE_ALL_MS_EXT_DEPRECATION_WARNINGS to suppress this warning.")]]
#else // ^^^ warning enabled / warning disabled vvv
#define _DEPRECATE_TR1_RANDOM
#endif // ^^^ warning disabled ^^^

#if _HAS_CXX20 && defined(__cpp_char8_t) && !defined(_SILENCE_CXX20_CODECVT_CHAR8_T_FACETS_DEPRECATION_WARNING) \
    && !defined(_SILENCE_ALL_CXX20_DEPRECATION_WARNINGS)
#define _CXX20_DEPRECATE_CODECVT_CHAR8_T_FACETS                                                                     \
    [[deprecated(                                                                                                   \
        "warning STL4047: std::codecvt<char16_t, char8_t, mbstate_t>, std::codecvt<char32_t, char8_t, mbstate_t>, " \
        "std::codecvt_byname<char16_t, char8_t, mbstate_t>, and std::codecvt_byname<char32_t, char8_t, mbstate_t> " \
        "are deprecated by LWG-3767. You can define _SILENCE_CXX20_CODECVT_CHAR8_T_FACETS_DEPRECATION_WARNING or "  \
        "_SILENCE_ALL_CXX20_DEPRECATION_WARNINGS to suppress this warning.")]]
#else // ^^^ warning enabled / warning disabled vvv
#define _CXX20_DEPRECATE_CODECVT_CHAR8_T_FACETS
#endif // ^^^ warning disabled ^^^

#if !defined(_SILENCE_LOCALE_EMPTY_DEPRECATION_WARNING) && !defined(_SILENCE_ALL_MS_EXT_DEPRECATION_WARNINGS)
#define _DEPRECATE_LOCALE_EMPTY                                                                                        \
    [[deprecated(                                                                                                      \
        "warning STL4048: locale::empty() is a non-Standard extension and will be removed in the future. A "           \
        "default-constructed locale can be used instead. You can define _SILENCE_LOCALE_EMPTY_DEPRECATION_WARNING or " \
        "_SILENCE_ALL_MS_EXT_DEPRECATION_WARNINGS to suppress this warning.")]]
#else // ^^^ warning enabled / warning disabled vvv
#define _DEPRECATE_LOCALE_EMPTY
#endif // ^^^ warning disabled ^^^

// next warning number: STL4049

// next error number: STL1009

// P0619R4 Removing C++17-Deprecated Features
#ifndef _HAS_FEATURES_REMOVED_IN_CXX20
#define _HAS_FEATURES_REMOVED_IN_CXX20 (!_HAS_CXX20)
#endif // !defined(_HAS_FEATURES_REMOVED_IN_CXX20)

#ifndef _HAS_DEPRECATED_ADAPTOR_TYPEDEFS
#define _HAS_DEPRECATED_ADAPTOR_TYPEDEFS (_HAS_FEATURES_REMOVED_IN_CXX20)
#endif // !defined(_HAS_DEPRECATED_ADAPTOR_TYPEDEFS)

#ifndef _HAS_DEPRECATED_ALLOCATOR_MEMBERS
#define _HAS_DEPRECATED_ALLOCATOR_MEMBERS (_HAS_FEATURES_REMOVED_IN_CXX20)
#endif // !defined(_HAS_DEPRECATED_ALLOCATOR_MEMBERS)

#ifndef _HAS_DEPRECATED_ALLOCATOR_VOID
#define _HAS_DEPRECATED_ALLOCATOR_VOID (_HAS_FEATURES_REMOVED_IN_CXX20)
#endif // !defined(_HAS_DEPRECATED_ALLOCATOR_VOID)

#ifndef _HAS_DEPRECATED_IS_LITERAL_TYPE
#define _HAS_DEPRECATED_IS_LITERAL_TYPE (_HAS_FEATURES_REMOVED_IN_CXX20)
#endif // !defined(_HAS_DEPRECATED_IS_LITERAL_TYPE)

#ifndef _HAS_DEPRECATED_NEGATORS
#define _HAS_DEPRECATED_NEGATORS (_HAS_FEATURES_REMOVED_IN_CXX20)
#endif // !defined(_HAS_DEPRECATED_NEGATORS)

#ifndef _HAS_DEPRECATED_RAW_STORAGE_ITERATOR
#define _HAS_DEPRECATED_RAW_STORAGE_ITERATOR (_HAS_FEATURES_REMOVED_IN_CXX20)
#endif // !defined(_HAS_DEPRECATED_RAW_STORAGE_ITERATOR)

#ifndef _HAS_DEPRECATED_RESULT_OF
#define _HAS_DEPRECATED_RESULT_OF (_HAS_FEATURES_REMOVED_IN_CXX20)
#endif // !defined(_HAS_DEPRECATED_RESULT_OF)

#ifndef _HAS_DEPRECATED_SHARED_PTR_UNIQUE
#define _HAS_DEPRECATED_SHARED_PTR_UNIQUE (_HAS_FEATURES_REMOVED_IN_CXX20)
#endif // !defined(_HAS_DEPRECATED_SHARED_PTR_UNIQUE)

#ifndef _HAS_DEPRECATED_TEMPORARY_BUFFER
#define _HAS_DEPRECATED_TEMPORARY_BUFFER (_HAS_FEATURES_REMOVED_IN_CXX20)
#endif // !defined(_HAS_DEPRECATED_TEMPORARY_BUFFER)

#ifndef _HAS_DEPRECATED_UNCAUGHT_EXCEPTION
#define _HAS_DEPRECATED_UNCAUGHT_EXCEPTION (_HAS_FEATURES_REMOVED_IN_CXX20)
#endif // !defined(_HAS_DEPRECATED_UNCAUGHT_EXCEPTION)

#if _HAS_DEPRECATED_ADAPTOR_TYPEDEFS
#define _ARGUMENT_TYPE_NAME        argument_type
#define _FIRST_ARGUMENT_TYPE_NAME  first_argument_type
#define _SECOND_ARGUMENT_TYPE_NAME second_argument_type
#define _RESULT_TYPE_NAME          result_type
#else // ^^^ _HAS_DEPRECATED_ADAPTOR_TYPEDEFS / !_HAS_DEPRECATED_ADAPTOR_TYPEDEFS vvv
#define _ARGUMENT_TYPE_NAME        _Unnameable_argument
#define _FIRST_ARGUMENT_TYPE_NAME  _Unnameable_first_argument
#define _SECOND_ARGUMENT_TYPE_NAME _Unnameable_second_argument
#define _RESULT_TYPE_NAME          _Unnameable_result
#endif // ^^^ !_HAS_DEPRECATED_ADAPTOR_TYPEDEFS ^^^

// P1423R3 char8_t Backward Compatibility Remediation
// Controls whether we allow the stream insertions this proposal forbids
#ifndef _HAS_STREAM_INSERTION_OPERATORS_DELETED_IN_CXX20
#define _HAS_STREAM_INSERTION_OPERATORS_DELETED_IN_CXX20 (_HAS_FEATURES_REMOVED_IN_CXX20)
#endif // !defined(_HAS_STREAM_INSERTION_OPERATORS_DELETED_IN_CXX20)

#ifndef _HAS_FEATURES_REMOVED_IN_CXX23
#define _HAS_FEATURES_REMOVED_IN_CXX23 (!_HAS_CXX23)
#endif // !defined(_HAS_FEATURES_REMOVED_IN_CXX23)

// P2186R2 Removing Garbage Collection Support
#ifndef _HAS_GARBAGE_COLLECTION_SUPPORT_DELETED_IN_CXX23
#define _HAS_GARBAGE_COLLECTION_SUPPORT_DELETED_IN_CXX23 (_HAS_FEATURES_REMOVED_IN_CXX23)
#endif // !defined(_HAS_GARBAGE_COLLECTION_SUPPORT_DELETED_IN_CXX23)

// C++14
#define __cpp_lib_chrono_udls                      201304L
#define __cpp_lib_complex_udls                     201309L
#define __cpp_lib_exchange_function                201304L
#define __cpp_lib_freestanding_algorithm           202311L
#define __cpp_lib_freestanding_array               202311L
#define __cpp_lib_freestanding_char_traits         202306L
#define __cpp_lib_freestanding_cstdlib             202306L
#define __cpp_lib_freestanding_cstring             202311L
#define __cpp_lib_freestanding_cwchar              202306L
#define __cpp_lib_freestanding_errc                202306L
#define __cpp_lib_freestanding_feature_test_macros 202306L
#define __cpp_lib_freestanding_functional          202306L
#define __cpp_lib_freestanding_iterator            202306L
#define __cpp_lib_freestanding_memory              202306L
#define __cpp_lib_freestanding_operator_new        202306L
#define __cpp_lib_freestanding_ratio               202306L
#define __cpp_lib_freestanding_tuple               202306L
#define __cpp_lib_freestanding_utility             202306L
#define __cpp_lib_generic_associative_lookup       201304L
#define __cpp_lib_integer_sequence                 201304L
#define __cpp_lib_integral_constant_callable       201304L
#define __cpp_lib_is_final                         201402L
#define __cpp_lib_is_null_pointer                  201309L
#define __cpp_lib_make_reverse_iterator            201402L
#define __cpp_lib_make_unique                      201304L
#define __cpp_lib_null_iterators                   201304L
#define __cpp_lib_quoted_string_io                 201304L
#define __cpp_lib_result_of_sfinae                 201210L
#define __cpp_lib_robust_nonmodifying_seq_ops      201304L
#ifndef _M_CEE_PURE
#define __cpp_lib_shared_timed_mutex 201402L
#endif // !defined(_M_CEE_PURE)
#define __cpp_lib_string_udls                  201304L
#define __cpp_lib_transformation_trait_aliases 201304L
#define __cpp_lib_tuple_element_t              201402L
#define __cpp_lib_tuples_by_type               201304L

// C++17
#define __cpp_lib_addressof_constexpr              201603L
#define __cpp_lib_allocator_traits_is_always_equal 201411L
#define __cpp_lib_as_const                         201510L
#define __cpp_lib_bool_constant                    201505L
#define __cpp_lib_enable_shared_from_this          201603L
#define __cpp_lib_incomplete_container_elements    201505L
#define __cpp_lib_invoke                           201411L
#define __cpp_lib_logical_traits                   201510L
#define __cpp_lib_map_try_emplace                  201411L
#define __cpp_lib_nonmember_container_access       201411L
#define __cpp_lib_shared_mutex                     201505L
#define __cpp_lib_transparent_operators            201510L
#define __cpp_lib_type_trait_variable_templates    201510L
#define __cpp_lib_uncaught_exceptions              201411L
#define __cpp_lib_unordered_map_try_emplace        201411L
#define __cpp_lib_void_t                           201411L

#if _HAS_CXX17
#if _HAS_STATIC_RTTI
#define __cpp_lib_any 201606L
#endif // _HAS_STATIC_RTTI
#define __cpp_lib_apply                      201603L
#define __cpp_lib_atomic_is_always_lock_free 201603L
#define __cpp_lib_boyer_moore_searcher       201603L
#if _HAS_STD_BYTE
#define __cpp_lib_byte 201603L
#endif // _HAS_STD_BYTE
#define __cpp_lib_clamp                             201603L
#define __cpp_lib_filesystem                        201703L
#define __cpp_lib_freestanding_charconv             202306L
#define __cpp_lib_freestanding_optional             202311L
#define __cpp_lib_freestanding_string_view          202311L
#define __cpp_lib_freestanding_variant              202311L
#define __cpp_lib_gcd_lcm                           201606L
#define __cpp_lib_hardware_interference_size        201703L
#define __cpp_lib_has_unique_object_representations 201606L
#define __cpp_lib_hypot                             201603L
#define __cpp_lib_is_aggregate                      201703L
#define __cpp_lib_is_invocable                      201703L
#define __cpp_lib_is_swappable                      201603L
#define __cpp_lib_launder                           201606L
#define __cpp_lib_make_from_tuple                   201606L
#define __cpp_lib_math_special_functions            201603L
#define __cpp_lib_memory_resource                   201603L
#define __cpp_lib_node_extract                      201606L
#define __cpp_lib_not_fn                            201603L
#ifndef _M_CEE_PURE
#define __cpp_lib_parallel_algorithm 201603L
#endif // !defined(_M_CEE_PURE)
#define __cpp_lib_raw_memory_algorithms 201606L
#define __cpp_lib_sample                201603L
#define __cpp_lib_scoped_lock           201703L
#define __cpp_lib_shared_ptr_weak_type  201606L
#define __cpp_lib_string_view           201803L
#define __cpp_lib_to_chars              201611L
#endif // _HAS_CXX17

// C++20
#define __cpp_lib_atomic_value_initialization 201911L

#ifdef __cpp_char8_t
#define __cpp_lib_char8_t 201907L
#endif // defined(__cpp_char8_t)

#ifdef __cpp_impl_coroutine
#define __cpp_lib_coroutine 201902L
#endif // defined(__cpp_impl_coroutine)

#if _HAS_CXX20
#define __cpp_lib_algorithm_iterator_requirements 202207L
#define __cpp_lib_assume_aligned                  201811L
#define __cpp_lib_atomic_flag_test                201907L
#define __cpp_lib_atomic_float                    201711L
#define __cpp_lib_atomic_lock_free_type_aliases   201907L
#define __cpp_lib_atomic_ref                      201806L
#define __cpp_lib_atomic_shared_ptr               201711L
#define __cpp_lib_atomic_wait                     201907L
#define __cpp_lib_barrier                         202302L
#define __cpp_lib_bind_front                      201907L
#define __cpp_lib_bit_cast                        201806L
#define __cpp_lib_bitops                          201907L
#define __cpp_lib_bounded_array_traits            201902L
#define __cpp_lib_common_reference                202302L
#define __cpp_lib_common_reference_wrapper        202302L
#define __cpp_lib_constexpr_algorithms            201806L
#define __cpp_lib_constexpr_complex               201711L
#define __cpp_lib_constexpr_dynamic_alloc         201907L
#define __cpp_lib_constexpr_functional            201907L
#define __cpp_lib_constexpr_iterator              201811L
#define __cpp_lib_constexpr_numeric               201911L
#define __cpp_lib_constexpr_string                201907L
#define __cpp_lib_constexpr_string_view           201811L
#define __cpp_lib_constexpr_tuple                 201811L
#define __cpp_lib_constexpr_utility               201811L
#define __cpp_lib_constexpr_vector                201907L
#define __cpp_lib_destroying_delete               201806L
#define __cpp_lib_endian                          201907L
#define __cpp_lib_erase_if                        202002L
#define __cpp_lib_format                          202304L
#define __cpp_lib_format_uchar                    202311L
#define __cpp_lib_freestanding_ranges             202306L
#define __cpp_lib_generic_unordered_lookup        201811L
#define __cpp_lib_int_pow2                        202002L
#define __cpp_lib_integer_comparison_functions    202002L
#define __cpp_lib_interpolate                     201902L
#define __cpp_lib_is_constant_evaluated           201811L

#ifndef __clang__ // TRANSITION, LLVM-48860
#define __cpp_lib_is_layout_compatible 201907L
#endif // ^^^ no workaround ^^^

#define __cpp_lib_is_nothrow_convertible 201806L

#ifndef __clang__ // TRANSITION, LLVM-48860
#define __cpp_lib_is_pointer_interconvertible 201907L
#endif // ^^^ no workaround ^^^

#define __cpp_lib_jthread                 201911L
#define __cpp_lib_latch                   201907L
#define __cpp_lib_list_remove_return_type 201806L
#define __cpp_lib_math_constants          201907L

#if !defined(__clang__) && !defined(__EDG__) // TRANSITION, Clang and EDG support for modules
#define __cpp_lib_modules 202207L
#endif // ^^^ no workaround ^^^

#define __cpp_lib_move_iterator_concept   202207L
#define __cpp_lib_polymorphic_allocator   201902L
#define __cpp_lib_remove_cvref            201711L
#define __cpp_lib_semaphore               201907L
#define __cpp_lib_smart_ptr_for_overwrite 202002L
#define __cpp_lib_source_location         201907L
#define __cpp_lib_span                    202002L
#define __cpp_lib_ssize                   201902L
#define __cpp_lib_starts_ends_with        201711L
#define __cpp_lib_syncbuf                 201803L
#define __cpp_lib_three_way_comparison    201907L
#define __cpp_lib_to_address              201711L
#define __cpp_lib_to_array                201907L
#define __cpp_lib_type_identity           201806L
#define __cpp_lib_unwrap_ref              201811L
#endif // _HAS_CXX20

// C++23
#if _HAS_CXX23
#define __cpp_lib_adaptor_iterator_pair_constructor 202106L
#define __cpp_lib_allocate_at_least                 202302L
#define __cpp_lib_associative_heterogeneous_erasure 202110L
#define __cpp_lib_bind_back                         202202L
#define __cpp_lib_byteswap                          202110L
#define __cpp_lib_constexpr_bitset                  202207L
#define __cpp_lib_constexpr_charconv                202207L
#define __cpp_lib_constexpr_typeinfo                202106L
#define __cpp_lib_containers_ranges                 202202L
#define __cpp_lib_expected                          202211L
#define __cpp_lib_format_ranges                     202207L
#define __cpp_lib_formatters                        202302L
#define __cpp_lib_forward_like                      202207L
#define __cpp_lib_freestanding_expected             202311L
#define __cpp_lib_freestanding_mdspan               202311L
#define __cpp_lib_generator                         202207L
#define __cpp_lib_invoke_r                          202106L
#define __cpp_lib_ios_noreplace                     202207L
#define __cpp_lib_is_scoped_enum                    202011L
#define __cpp_lib_mdspan                            202207L
#define __cpp_lib_move_only_function                202110L
#define __cpp_lib_out_ptr                           202311L
#define __cpp_lib_print                             202406L
#define __cpp_lib_ranges_as_const                   202311L
#define __cpp_lib_ranges_as_rvalue                  202207L
#define __cpp_lib_ranges_cartesian_product          202207L
#define __cpp_lib_ranges_chunk                      202202L
#define __cpp_lib_ranges_chunk_by                   202202L
#define __cpp_lib_ranges_contains                   202207L
#define __cpp_lib_ranges_enumerate                  202302L
#define __cpp_lib_ranges_find_last                  202207L
#define __cpp_lib_ranges_fold                       202207L
#define __cpp_lib_ranges_iota                       202202L
#define __cpp_lib_ranges_join_with                  202202L
#define __cpp_lib_ranges_repeat                     202207L
#define __cpp_lib_ranges_slide                      202202L
#define __cpp_lib_ranges_starts_ends_with           202106L
#define __cpp_lib_ranges_stride                     202207L
#define __cpp_lib_ranges_to_container               202202L
#define __cpp_lib_ranges_zip                        202110L
#define __cpp_lib_spanstream                        202106L
#define __cpp_lib_stacktrace                        202011L
#define __cpp_lib_stdatomic_h                       202011L
#define __cpp_lib_string_contains                   202011L
#define __cpp_lib_string_resize_and_overwrite       202110L
#define __cpp_lib_to_underlying                     202102L
#define __cpp_lib_tuple_like                        202207L
#define __cpp_lib_unreachable                       202202L
#endif // _HAS_CXX23

// macros with language mode sensitivity
#if _HAS_CXX20
#define __cpp_lib_array_constexpr 201811L // P1032R1 Miscellaneous constexpr
#elif _HAS_CXX17
#define __cpp_lib_array_constexpr 201803L // P0858R0 Constexpr Iterator Requirements
#endif

#if _HAS_CXX20
#define __cpp_lib_chrono 201907L // P1466R3 Miscellaneous Minor Fixes For <chrono>
#elif _HAS_CXX17
#define __cpp_lib_chrono 201611L // P0505R0 constexpr For <chrono> (Again)
#else
#define __cpp_lib_chrono 201510L // P0092R1 <chrono> floor(), ceil(), round(), abs()
#endif

#if _HAS_CXX23
#define __cpp_lib_concepts 202207L // P2404R3 Move-Only Types For Comparison Concepts
#elif _HAS_CXX20
#define __cpp_lib_concepts 202002L // P1964R2 Replacing boolean With boolean-testable
#endif

#if _HAS_CXX23
#define __cpp_lib_constexpr_memory 202202L // P2273R3 constexpr unique_ptr
#elif _HAS_CXX20
#define __cpp_lib_constexpr_memory 201811L // P1006R1 constexpr For pointer_traits<T*>::pointer_to()
#endif

#ifndef _M_CEE_PURE
#if _HAS_CXX20
#define __cpp_lib_execution 201902L // P1001R2 execution::unseq
#elif _HAS_CXX17
#define __cpp_lib_execution 201603L // P0024R2 Parallel Algorithms
#endif // language mode
#endif // !defined(_M_CEE_PURE)

#if _HAS_CXX23
#define __cpp_lib_optional 202110L // P0798R8 Monadic Operations For optional
#elif _HAS_CXX20
#define __cpp_lib_optional 202106L // P2231R1 Completing constexpr In optional And variant
#elif _HAS_CXX17
#define __cpp_lib_optional 201606L // P0307R2 Making Optional Greater Equal Again
#endif

#if _HAS_CXX23
// P2997R1 Removing The Common Reference Requirement From The Indirectly Invocable Concepts
#define __cpp_lib_ranges 202406L
#elif _HAS_CXX20
#define __cpp_lib_ranges 202110L // P2415R2 What Is A view?
#endif

#if _HAS_CXX20
#define __cpp_lib_shared_ptr_arrays 201707L // P0674R1 make_shared() For Arrays
#else
#define __cpp_lib_shared_ptr_arrays 201611L // P0497R0 Fixing shared_ptr For Arrays
#endif

#if _HAS_CXX23
#define __cpp_lib_shift 202202L // P2440R1 ranges::shift_left, ranges::shift_right
#elif _HAS_CXX20
#define __cpp_lib_shift 201806L // P0769R2 shift_left(), shift_right()
#endif

#if _HAS_CXX20
#define __cpp_lib_variant 202106L // P2231R1 Completing constexpr In optional And variant
#elif _HAS_CXX17
#define __cpp_lib_variant 202102L // P2162R2 Inheriting From variant
#endif

#define __cpp_lib_experimental_filesystem 201406L

#ifdef _RTC_CONVERSION_CHECKS_ENABLED
#ifndef _ALLOW_RTCc_IN_STL
#error /RTCc rejects conformant code, so it is not supported by the C++ Standard Library. Either remove this \
compiler option, or define _ALLOW_RTCc_IN_STL to suppress this error.
#endif // !defined(_ALLOW_RTCc_IN_STL)
#endif // defined(_RTC_CONVERSION_CHECKS_ENABLED)

#define _EMPTY_ARGUMENT // for empty macro argument

// extern "C++" attaches declarations to the global module, see N4964 [module.unit]/7.2.
// It has no effect in C++14/17.

// In the STL's headers (which might be used to build the named module std), we unconditionally
// and directly mark declarations of our separately compiled machinery as extern "C++", allowing
// the named module to work with the separately compiled code (which is always built classically).

// TRANSITION: _USE_EXTERN_CXX_EVERYWHERE_FOR_STL controls whether we also wrap the STL's
// header-only code in this linkage-specification, as a temporary workaround to allow
// importing the named module in a translation unit with classic includes.

#ifndef _USE_EXTERN_CXX_EVERYWHERE_FOR_STL
#define _USE_EXTERN_CXX_EVERYWHERE_FOR_STL _HAS_CXX20
#endif // ^^^ !defined(_USE_EXTERN_CXX_EVERYWHERE_FOR_STL) ^^^

#if _USE_EXTERN_CXX_EVERYWHERE_FOR_STL
#define _EXTERN_CXX_WORKAROUND     extern "C++" {
#define _END_EXTERN_CXX_WORKAROUND }
#else // ^^^ _USE_EXTERN_CXX_EVERYWHERE_FOR_STL / !_USE_EXTERN_CXX_EVERYWHERE_FOR_STL vvv
#define _EXTERN_CXX_WORKAROUND
#define _END_EXTERN_CXX_WORKAROUND
#endif // ^^^ !_USE_EXTERN_CXX_EVERYWHERE_FOR_STL ^^^

#define _STD_BEGIN         \
    _EXTERN_CXX_WORKAROUND \
    namespace std {
#define _STD_END \
    }            \
    _END_EXTERN_CXX_WORKAROUND

#define _STD    ::std::
#define _CHRONO ::std::chrono::
#define _RANGES ::std::ranges::

// We use the stdext (standard extension) namespace to contain non-standard extensions
#pragma push_macro("stdext")
#undef stdext
#define _STDEXT_BEGIN      \
    _EXTERN_CXX_WORKAROUND \
    namespace stdext {
#define _STDEXT_END \
    }               \
    _END_EXTERN_CXX_WORKAROUND

#define _STDEXT ::stdext::
#pragma pop_macro("stdext")

#define _CSTD ::

#ifdef _M_CEE_PURE
#define _EXTERN_C_UNLESS_PURE
#define _END_EXTERN_C_UNLESS_PURE
#else // ^^^ defined(_M_CEE_PURE) / !defined(_M_CEE_PURE) vvv
#define _EXTERN_C_UNLESS_PURE     extern "C" {
#define _END_EXTERN_C_UNLESS_PURE } // extern "C"
#endif // ^^^ !defined(_M_CEE_PURE) ^^^

#if defined(MRTDLL) && !defined(_CRTBLD)
#error In yvals_core.h, defined(MRTDLL) implies defined(_CRTBLD); !defined(_CRTBLD) implies !defined(MRTDLL)
#endif // defined(MRTDLL) && !defined(_CRTBLD)

#if defined(MRTDLL) && !defined(_M_CEE_PURE)
#error In yvals_core.h, defined(MRTDLL) implies defined(_M_CEE_PURE); !defined(_M_CEE_PURE) implies !defined(MRTDLL)
#endif // defined(MRTDLL) && !defined(_M_CEE_PURE)

#define _STL_WIN32_WINNT_VISTA 0x0600 // _WIN32_WINNT_VISTA from sdkddkver.h
#define _STL_WIN32_WINNT_WIN7  0x0601 // _WIN32_WINNT_WIN7 from sdkddkver.h
#define _STL_WIN32_WINNT_WIN8  0x0602 // _WIN32_WINNT_WIN8 from sdkddkver.h
#define _STL_WIN32_WINNT_WIN10 0x0A00 // _WIN32_WINNT_WIN10 from sdkddkver.h

// Note that the STL DLL builds will set this to XP for ABI compatibility with VS2015 which supported XP.
#ifndef _STL_WIN32_WINNT
#if defined(_M_ARM64)
// The first ARM64 Windows was Windows 10
#define _STL_WIN32_WINNT _STL_WIN32_WINNT_WIN10
#elif defined(_M_ARM) || defined(_ONECORE) || defined(_CRT_APP)
// The first ARM or OneCore or App Windows was Windows 8
#define _STL_WIN32_WINNT _STL_WIN32_WINNT_WIN8
#else // ^^^ default to Win8 / default to Win7 vvv
// The earliest Windows supported by this implementation is Windows 7
#define _STL_WIN32_WINNT _STL_WIN32_WINNT_WIN7
#endif // ^^^ !defined(_M_ARM) && !defined(_M_ARM64) && !defined(_ONECORE) && !defined(_CRT_APP) ^^^
#endif // !defined(_STL_WIN32_WINNT)

#ifdef __cpp_noexcept_function_type
#define _NOEXCEPT_FNPTR noexcept
#else // ^^^ defined(__cpp_noexcept_function_type) / !defined(__cpp_noexcept_function_type) vvv
#define _NOEXCEPT_FNPTR
#endif // ^^^ !defined(__cpp_noexcept_function_type) ^^^

#ifdef __clang__
#define _STL_INTRIN_HEADER <intrin.h>
#define _STL_UNREACHABLE   __builtin_unreachable()
#else // ^^^ defined(__clang__) / !defined(__clang__) vvv
#define _STL_INTRIN_HEADER <intrin0.h>
#define _STL_UNREACHABLE   __assume(false)
#endif // ^^^ !defined(__clang__) ^^^

#ifdef _ENABLE_STL_INTERNAL_CHECK
#define _STL_INTERNAL_STATIC_ASSERT(...) static_assert(__VA_ARGS__, #__VA_ARGS__)
#else // ^^^ defined(_ENABLE_STL_INTERNAL_CHECK) / !defined(_ENABLE_STL_INTERNAL_CHECK) vvv
#define _STL_INTERNAL_STATIC_ASSERT(...)
#endif // ^^^ !defined(_ENABLE_STL_INTERNAL_CHECK) ^^^

#if defined(__CUDACC__) || (defined(__clang__) && __clang_major__ < 16)
// TRANSITION, CUDA 12.4 doesn't have downlevel support for static call operators.
// TRANSITION, VSO-2397560, temporary workaround for Real World Code relying on ancient Clang versions.
#define _STATIC_CALL_OPERATOR
#define _CONST_CALL_OPERATOR const
#define _STATIC_LAMBDA
#elif defined(__clang__) || defined(__EDG__) // no workaround
#define _STATIC_CALL_OPERATOR static
#define _CONST_CALL_OPERATOR
#define _STATIC_LAMBDA static
#else // TRANSITION, VSO-2383148, fixed in VS 2022 17.14 Preview 3
#define _STATIC_CALL_OPERATOR static
#define _CONST_CALL_OPERATOR
#define _STATIC_LAMBDA
#endif // ^^^ workaround ^^^

#ifdef __CUDACC__ // TRANSITION, CUDA 12.4 doesn't recognize MSVC __restrict; CUDA __restrict__ is not usable in C++
#define _RESTRICT
#else // ^^^ defined(__CUDACC__) / !defined(__CUDACC__) vvv
#define _RESTRICT __restrict
#endif // ^^^ !defined(__CUDACC__) ^^^

#endif // _STL_COMPILER_PREPROCESSOR
#endif // _YVALS_CORE_H_
