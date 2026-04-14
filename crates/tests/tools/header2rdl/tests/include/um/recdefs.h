//--------------------------------------------------------------------------
//  This is part of the Microsoft Tablet PC Platform SDK
//  Copyright (C) 2002 Microsoft Corporation
//  All rights reserved.
//
//
// Module:       
//      RecDefs.h
//
//--------------------------------------------------------------------------


#ifndef __INC_RECDEFS_H
#define __INC_RECDEFS_H

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

// RECO FLAGS
#define RECOFLAG_WORDMODE		0x00000001
#define RECOFLAG_COERCE			0x00000002
#define RECOFLAG_SINGLESEG		0x00000004
#define RECOFLAG_PREFIXOK		0x00000008
#define RECOFLAG_LINEMODE		0x00000010
#define RECOFLAG_DISABLEPERSONALIZATION 0x00000020
#define RECOFLAG_AUTOSPACE		0x00000040

// Confidence constants
#define RECOCONF_LOWCONFIDENCE -1
#define RECOCONF_MEDIUMCONFIDENCE 0
#define RECOCONF_HIGHCONFIDENCE 1
#define RECOCONF_NOTSET    128

// Gesture ids
#define GESTURE_NULL                        0xf000
#define GESTURE_SCRATCHOUT                  0xf001
#define GESTURE_TRIANGLE                    0xf002
#define GESTURE_SQUARE                      0xf003
#define GESTURE_STAR                        0xf004
#define GESTURE_CHECK                       0xf005
#define GESTURE_INFINITY                    0xf006
#define GESTURE_CROSS                       0xf007
#define GESTURE_PARAGRAPH                   0xf008
#define GESTURE_SECTION                     0xf009
#define GESTURE_BULLET                      0xf00a
#define GESTURE_BULLET_CROSS                0xf00b
#define GESTURE_SQUIGGLE                    0xf00c
#define GESTURE_SWAP                        0xf00d
#define GESTURE_OPENUP                      0xf00e
#define GESTURE_CLOSEUP                     0xf00f
#define GESTURE_CURLICUE                    0xf010
#define GESTURE_DOUBLE_CURLICUE             0xf011
#define GESTURE_RECTANGLE                   0xf012
#define GESTURE_CIRCLE                      0xf020
#define GESTURE_DOUBLE_CIRCLE               0xf021
#define GESTURE_CIRCLE_TAP                  0xf022
#define GESTURE_CIRCLE_CIRCLE               0xf023
#define GESTURE_CIRCLE_CROSS                0xf025
#define GESTURE_CIRCLE_LINE_VERT            0xf026
#define GESTURE_CIRCLE_LINE_HORZ            0xf027
#define GESTURE_SEMICIRCLE_LEFT             0xf028
#define GESTURE_SEMICIRCLE_RIGHT            0xf029
#define GESTURE_CHEVRON_UP                  0xf030
#define GESTURE_CHEVRON_DOWN                0xf031
#define GESTURE_CHEVRON_LEFT                0xf032
#define GESTURE_CHEVRON_RIGHT               0xf033
#define GESTURE_ARROW_UP                    0xf038
#define GESTURE_ARROW_DOWN                  0xf039
#define GESTURE_ARROW_LEFT                  0xf03a
#define GESTURE_ARROW_RIGHT                 0xf03b
#define GESTURE_DOUBLE_ARROW_UP             0xf03c
#define GESTURE_DOUBLE_ARROW_DOWN           0xf03d
#define GESTURE_DOUBLE_ARROW_LEFT           0xf03e
#define GESTURE_DOUBLE_ARROW_RIGHT          0xf03f
#define GESTURE_UP_ARROW_LEFT               0xf040
#define GESTURE_UP_ARROW_RIGHT              0xf041
#define GESTURE_DOWN_ARROW_LEFT             0xf042
#define GESTURE_DOWN_ARROW_RIGHT            0xf043
#define GESTURE_LEFT_ARROW_UP               0xf044
#define GESTURE_LEFT_ARROW_DOWN             0xf045
#define GESTURE_RIGHT_ARROW_UP              0xf046
#define GESTURE_RIGHT_ARROW_DOWN            0xf047
#define GESTURE_UP                          0xf058
#define GESTURE_DOWN                        0xf059
#define GESTURE_LEFT                        0xf05a
#define GESTURE_RIGHT                       0xf05b
#define GESTURE_DIAGONAL_LEFTUP             0xf05c
#define GESTURE_DIAGONAL_RIGHTUP            0xf05d
#define GESTURE_DIAGONAL_LEFTDOWN           0xf05e
#define GESTURE_DIAGONAL_RIGHTDOWN          0xf05f
#define GESTURE_UP_DOWN                     0xf060
#define GESTURE_DOWN_UP                     0xf061
#define GESTURE_LEFT_RIGHT                  0xf062
#define GESTURE_RIGHT_LEFT                  0xf063
#define GESTURE_UP_LEFT_LONG                0xf064
#define GESTURE_UP_RIGHT_LONG               0xf065
#define GESTURE_DOWN_LEFT_LONG              0xf066
#define GESTURE_DOWN_RIGHT_LONG             0xf067
#define GESTURE_UP_LEFT                     0xf068
#define GESTURE_UP_RIGHT                    0xf069
#define GESTURE_DOWN_LEFT                   0xf06a
#define GESTURE_DOWN_RIGHT                  0xf06b
#define GESTURE_LEFT_UP                     0xf06c
#define GESTURE_LEFT_DOWN                   0xf06d
#define GESTURE_RIGHT_UP                    0xf06e
#define GESTURE_RIGHT_DOWN                  0xf06f
#define GESTURE_LETTER_A                    0xf080
#define GESTURE_LETTER_B                    0xf081
#define GESTURE_LETTER_C                    0xf082
#define GESTURE_LETTER_D                    0xf083
#define GESTURE_LETTER_E                    0xf084
#define GESTURE_LETTER_F                    0xf085
#define GESTURE_LETTER_G                    0xf086
#define GESTURE_LETTER_H                    0xf087
#define GESTURE_LETTER_I                    0xf088
#define GESTURE_LETTER_J                    0xf089
#define GESTURE_LETTER_K                    0xf08a
#define GESTURE_LETTER_L                    0xf08b
#define GESTURE_LETTER_M                    0xf08c
#define GESTURE_LETTER_N                    0xf08d
#define GESTURE_LETTER_O                    0xf08e
#define GESTURE_LETTER_P                    0xf08f
#define GESTURE_LETTER_Q                    0xf090
#define GESTURE_LETTER_R                    0xf091
#define GESTURE_LETTER_S                    0xf092
#define GESTURE_LETTER_T                    0xf093
#define GESTURE_LETTER_U                    0xf094
#define GESTURE_LETTER_V                    0xf095
#define GESTURE_LETTER_W                    0xf096
#define GESTURE_LETTER_X                    0xf097
#define GESTURE_LETTER_Y                    0xf098
#define GESTURE_LETTER_Z                    0xf099
#define GESTURE_DIGIT_0                     0xf09a
#define GESTURE_DIGIT_1                     0xf09b
#define GESTURE_DIGIT_2                     0xf09c
#define GESTURE_DIGIT_3                     0xf09d
#define GESTURE_DIGIT_4                     0xf09e
#define GESTURE_DIGIT_5                     0xf09f
#define GESTURE_DIGIT_6                     0xf0a0
#define GESTURE_DIGIT_7                     0xf0a1
#define GESTURE_DIGIT_8                     0xf0a2
#define GESTURE_DIGIT_9                     0xf0a3
#define GESTURE_EXCLAMATION                 0xf0a4
#define GESTURE_QUESTION                    0xf0a5
#define GESTURE_SHARP                       0xf0a6
#define GESTURE_DOLLAR                      0xf0a7
#define GESTURE_ASTERISK                    0xf0a8
#define GESTURE_PLUS                        0xf0a9
#define GESTURE_DOUBLE_UP                   0xf0b8
#define GESTURE_DOUBLE_DOWN                 0xf0b9
#define GESTURE_DOUBLE_LEFT                 0xf0ba
#define GESTURE_DOUBLE_RIGHT                0xf0bb
#define GESTURE_TRIPLE_UP                   0xf0bc
#define GESTURE_TRIPLE_DOWN                 0xf0bd
#define GESTURE_TRIPLE_LEFT                 0xf0be
#define GESTURE_TRIPLE_RIGHT                0xf0bf
#define GESTURE_BRACKET_OVER                0xf0e4
#define GESTURE_BRACKET_UNDER               0xf0e5
#define GESTURE_BRACKET_LEFT                0xf0e6
#define GESTURE_BRACKET_RIGHT               0xf0e7
#define GESTURE_BRACE_OVER                  0xf0e8
#define GESTURE_BRACE_UNDER                 0xf0e9
#define GESTURE_BRACE_LEFT                  0xf0ea
#define GESTURE_BRACE_RIGHT                 0xf0eb
#define GESTURE_TAP                         0xf0f0
#define GESTURE_DOUBLE_TAP                  0xf0f1
#define GESTURE_TRIPLE_TAP                  0xf0f2
#define GESTURE_QUAD_TAP                    0xf0f3

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region App Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)

// RECO FLAGS
#define RECOFLAG_WORDMODE		0x00000001
#define RECOFLAG_COERCE			0x00000002
#define RECOFLAG_SINGLESEG		0x00000004
#define RECOFLAG_PREFIXOK		0x00000008
#define RECOFLAG_LINEMODE		0x00000010
#define RECOFLAG_DISABLEPERSONALIZATION 0x00000020
#define RECOFLAG_AUTOSPACE		0x00000040

// Confidence constants
#define RECOCONF_LOWCONFIDENCE -1
#define RECOCONF_MEDIUMCONFIDENCE 0
#define RECOCONF_HIGHCONFIDENCE 1
#define RECOCONF_NOTSET    128

// Gesture ids
#define GESTURE_NULL                        0xf000
#define GESTURE_SCRATCHOUT                  0xf001
#define GESTURE_TRIANGLE                    0xf002
#define GESTURE_SQUARE                      0xf003
#define GESTURE_STAR                        0xf004
#define GESTURE_CHECK                       0xf005
#define GESTURE_INFINITY                    0xf006
#define GESTURE_CROSS                       0xf007
#define GESTURE_PARAGRAPH                   0xf008
#define GESTURE_SECTION                     0xf009
#define GESTURE_BULLET                      0xf00a
#define GESTURE_BULLET_CROSS                0xf00b
#define GESTURE_SQUIGGLE                    0xf00c
#define GESTURE_SWAP                        0xf00d
#define GESTURE_OPENUP                      0xf00e
#define GESTURE_CLOSEUP                     0xf00f
#define GESTURE_CURLICUE                    0xf010
#define GESTURE_DOUBLE_CURLICUE             0xf011
#define GESTURE_RECTANGLE                   0xf012
#define GESTURE_CIRCLE                      0xf020
#define GESTURE_DOUBLE_CIRCLE               0xf021
#define GESTURE_CIRCLE_TAP                  0xf022
#define GESTURE_CIRCLE_CIRCLE               0xf023
#define GESTURE_CIRCLE_CROSS                0xf025
#define GESTURE_CIRCLE_LINE_VERT            0xf026
#define GESTURE_CIRCLE_LINE_HORZ            0xf027
#define GESTURE_SEMICIRCLE_LEFT             0xf028
#define GESTURE_SEMICIRCLE_RIGHT            0xf029
#define GESTURE_CHEVRON_UP                  0xf030
#define GESTURE_CHEVRON_DOWN                0xf031
#define GESTURE_CHEVRON_LEFT                0xf032
#define GESTURE_CHEVRON_RIGHT               0xf033
#define GESTURE_ARROW_UP                    0xf038
#define GESTURE_ARROW_DOWN                  0xf039
#define GESTURE_ARROW_LEFT                  0xf03a
#define GESTURE_ARROW_RIGHT                 0xf03b
#define GESTURE_DOUBLE_ARROW_UP             0xf03c
#define GESTURE_DOUBLE_ARROW_DOWN           0xf03d
#define GESTURE_DOUBLE_ARROW_LEFT           0xf03e
#define GESTURE_DOUBLE_ARROW_RIGHT          0xf03f
#define GESTURE_UP_ARROW_LEFT               0xf040
#define GESTURE_UP_ARROW_RIGHT              0xf041
#define GESTURE_DOWN_ARROW_LEFT             0xf042
#define GESTURE_DOWN_ARROW_RIGHT            0xf043
#define GESTURE_LEFT_ARROW_UP               0xf044
#define GESTURE_LEFT_ARROW_DOWN             0xf045
#define GESTURE_RIGHT_ARROW_UP              0xf046
#define GESTURE_RIGHT_ARROW_DOWN            0xf047
#define GESTURE_UP                          0xf058
#define GESTURE_DOWN                        0xf059
#define GESTURE_LEFT                        0xf05a
#define GESTURE_RIGHT                       0xf05b
#define GESTURE_DIAGONAL_LEFTUP             0xf05c
#define GESTURE_DIAGONAL_RIGHTUP            0xf05d
#define GESTURE_DIAGONAL_LEFTDOWN           0xf05e
#define GESTURE_DIAGONAL_RIGHTDOWN          0xf05f
#define GESTURE_UP_DOWN                     0xf060
#define GESTURE_DOWN_UP                     0xf061
#define GESTURE_LEFT_RIGHT                  0xf062
#define GESTURE_RIGHT_LEFT                  0xf063
#define GESTURE_UP_LEFT_LONG                0xf064
#define GESTURE_UP_RIGHT_LONG               0xf065
#define GESTURE_DOWN_LEFT_LONG              0xf066
#define GESTURE_DOWN_RIGHT_LONG             0xf067
#define GESTURE_UP_LEFT                     0xf068
#define GESTURE_UP_RIGHT                    0xf069
#define GESTURE_DOWN_LEFT                   0xf06a
#define GESTURE_DOWN_RIGHT                  0xf06b
#define GESTURE_LEFT_UP                     0xf06c
#define GESTURE_LEFT_DOWN                   0xf06d
#define GESTURE_RIGHT_UP                    0xf06e
#define GESTURE_RIGHT_DOWN                  0xf06f
#define GESTURE_LETTER_A                    0xf080
#define GESTURE_LETTER_B                    0xf081
#define GESTURE_LETTER_C                    0xf082
#define GESTURE_LETTER_D                    0xf083
#define GESTURE_LETTER_E                    0xf084
#define GESTURE_LETTER_F                    0xf085
#define GESTURE_LETTER_G                    0xf086
#define GESTURE_LETTER_H                    0xf087
#define GESTURE_LETTER_I                    0xf088
#define GESTURE_LETTER_J                    0xf089
#define GESTURE_LETTER_K                    0xf08a
#define GESTURE_LETTER_L                    0xf08b
#define GESTURE_LETTER_M                    0xf08c
#define GESTURE_LETTER_N                    0xf08d
#define GESTURE_LETTER_O                    0xf08e
#define GESTURE_LETTER_P                    0xf08f
#define GESTURE_LETTER_Q                    0xf090
#define GESTURE_LETTER_R                    0xf091
#define GESTURE_LETTER_S                    0xf092
#define GESTURE_LETTER_T                    0xf093
#define GESTURE_LETTER_U                    0xf094
#define GESTURE_LETTER_V                    0xf095
#define GESTURE_LETTER_W                    0xf096
#define GESTURE_LETTER_X                    0xf097
#define GESTURE_LETTER_Y                    0xf098
#define GESTURE_LETTER_Z                    0xf099
#define GESTURE_DIGIT_0                     0xf09a
#define GESTURE_DIGIT_1                     0xf09b
#define GESTURE_DIGIT_2                     0xf09c
#define GESTURE_DIGIT_3                     0xf09d
#define GESTURE_DIGIT_4                     0xf09e
#define GESTURE_DIGIT_5                     0xf09f
#define GESTURE_DIGIT_6                     0xf0a0
#define GESTURE_DIGIT_7                     0xf0a1
#define GESTURE_DIGIT_8                     0xf0a2
#define GESTURE_DIGIT_9                     0xf0a3
#define GESTURE_EXCLAMATION                 0xf0a4
#define GESTURE_QUESTION                    0xf0a5
#define GESTURE_SHARP                       0xf0a6
#define GESTURE_DOLLAR                      0xf0a7
#define GESTURE_ASTERISK                    0xf0a8
#define GESTURE_PLUS                        0xf0a9
#define GESTURE_DOUBLE_UP                   0xf0b8
#define GESTURE_DOUBLE_DOWN                 0xf0b9
#define GESTURE_DOUBLE_LEFT                 0xf0ba
#define GESTURE_DOUBLE_RIGHT                0xf0bb
#define GESTURE_TRIPLE_UP                   0xf0bc
#define GESTURE_TRIPLE_DOWN                 0xf0bd
#define GESTURE_TRIPLE_LEFT                 0xf0be
#define GESTURE_TRIPLE_RIGHT                0xf0bf
#define GESTURE_BRACKET_OVER                0xf0e4
#define GESTURE_BRACKET_UNDER               0xf0e5
#define GESTURE_BRACKET_LEFT                0xf0e6
#define GESTURE_BRACKET_RIGHT               0xf0e7
#define GESTURE_BRACE_OVER                  0xf0e8
#define GESTURE_BRACE_UNDER                 0xf0e9
#define GESTURE_BRACE_LEFT                  0xf0ea
#define GESTURE_BRACE_RIGHT                 0xf0eb
#define GESTURE_TAP                         0xf0f0
#define GESTURE_DOUBLE_TAP                  0xf0f1
#define GESTURE_TRIPLE_TAP                  0xf0f2
#define GESTURE_QUAD_TAP                    0xf0f3

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion

#endif /* Please keep the trailing blank line, otherwise you will hit RC1004: "unexpected end of file found" */
