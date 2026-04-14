// Copyright (C) Microsoft Corporation. All rights reserved.

#pragma once

#if defined(UM_NDIS689) || defined(NDIS_WRAPPER) || \
    (defined(NDIS_PLATFORM) && (defined(NDIS689) || defined(NDIS689_MINIPORT))) || \
    (NDIS_MINIPORT_MAJOR_VERSION == 6 && NDIS_MINIPORT_MINOR_VERSION == 89)
#define NDIS_SUPPORT_NDIS689 1
#endif

#if defined(UM_NDIS688) || defined(NDIS_WRAPPER) || \
    (defined(NDIS_PLATFORM) && (defined(NDIS688) || defined(NDIS688_MINIPORT))) || \
    (NDIS_MINIPORT_MAJOR_VERSION == 6 && NDIS_MINIPORT_MINOR_VERSION == 88)
#define NDIS_SUPPORT_NDIS688 1
#endif

#if defined(UM_NDIS687) || defined(NDIS_WRAPPER) || \
    (defined(NDIS_PLATFORM) && (defined(NDIS687) || defined(NDIS687_MINIPORT))) || \
    (NDIS_MINIPORT_MAJOR_VERSION == 6 && NDIS_MINIPORT_MINOR_VERSION == 87)
#define NDIS_SUPPORT_NDIS687 1
#endif

#if defined(UM_NDIS686) || defined(NDIS_WRAPPER) || \
    (defined(NDIS_PLATFORM) && (defined(NDIS686) || defined(NDIS686_MINIPORT))) || \
    (NDIS_MINIPORT_MAJOR_VERSION == 6 && NDIS_MINIPORT_MINOR_VERSION == 86)
#define NDIS_SUPPORT_NDIS686 1
#endif

#if defined(UM_NDIS685) || defined(NDIS_WRAPPER) || \
    (defined(NDIS_PLATFORM) && (defined(NDIS685) || defined(NDIS685_MINIPORT))) || \
    (NDIS_MINIPORT_MAJOR_VERSION == 6 && NDIS_MINIPORT_MINOR_VERSION == 85)
#define NDIS_SUPPORT_NDIS685 1
#endif

#if defined(UM_NDIS684) || defined(NDIS_WRAPPER) || \
    (defined(NDIS_PLATFORM) && (defined(NDIS684) || defined(NDIS684_MINIPORT))) || \
    (NDIS_MINIPORT_MAJOR_VERSION == 6 && NDIS_MINIPORT_MINOR_VERSION == 84)
#define NDIS_SUPPORT_NDIS684 1
#endif

#if defined(UM_NDIS683) || defined(NDIS_WRAPPER) || \
    (defined(NDIS_PLATFORM) && (defined(NDIS683) || defined(NDIS683_MINIPORT))) || \
    (NDIS_MINIPORT_MAJOR_VERSION == 6 && NDIS_MINIPORT_MINOR_VERSION == 83)
#define NDIS_SUPPORT_NDIS683 1
#endif

#if defined(UM_NDIS682) || defined(NDIS_WRAPPER) || \
    (defined(NDIS_PLATFORM) && (defined(NDIS682) || defined(NDIS682_MINIPORT))) || \
    (NDIS_MINIPORT_MAJOR_VERSION == 6 && NDIS_MINIPORT_MINOR_VERSION == 82)
#define NDIS_SUPPORT_NDIS682 1
#endif

#if defined(UM_NDIS681) || defined(NDIS_WRAPPER) || \
    (defined(NDIS_PLATFORM) && (defined(NDIS681) || defined(NDIS681_MINIPORT))) || \
    (NDIS_MINIPORT_MAJOR_VERSION == 6 && NDIS_MINIPORT_MINOR_VERSION == 81)
#define NDIS_SUPPORT_NDIS681 1
#endif

#if defined(UM_NDIS680) || defined(NDIS_WRAPPER) || \
    (defined(NDIS_PLATFORM) && (defined(NDIS680) || defined(NDIS680_MINIPORT))) || \
    (NDIS_MINIPORT_MAJOR_VERSION == 6 && NDIS_MINIPORT_MINOR_VERSION == 80)
#define NDIS_SUPPORT_NDIS680 1
#endif

#if defined(UM_NDIS670) || defined(NDIS_WRAPPER) || \
    (defined(NDIS_PLATFORM) && (defined(NDIS670) || defined(NDIS670_MINIPORT))) || \
    (NDIS_MINIPORT_MAJOR_VERSION == 6 && NDIS_MINIPORT_MINOR_VERSION == 70)
#define NDIS_SUPPORT_NDIS670 1
#endif

#if defined(UM_NDIS660) || defined(NDIS_WRAPPER) || \
    (defined(NDIS_PLATFORM) && (defined(NDIS660) || defined(NDIS660_MINIPORT))) || \
    (NDIS_MINIPORT_MAJOR_VERSION == 6 && NDIS_MINIPORT_MINOR_VERSION == 60)
#define NDIS_SUPPORT_NDIS660 1
#endif

#if defined(UM_NDIS651) || defined(NDIS_WRAPPER) || \
    (defined(NDIS_PLATFORM) && (defined(NDIS651) || defined(NDIS651_MINIPORT))) || \
    (NDIS_MINIPORT_MAJOR_VERSION == 6 && NDIS_MINIPORT_MINOR_VERSION == 51)
#define NDIS_SUPPORT_NDIS651 1
#endif

#if defined(UM_NDIS650) || defined(NDIS_WRAPPER) || \
    (defined(NDIS_PLATFORM) && (defined(NDIS650) || defined(NDIS650_MINIPORT))) || \
    (NDIS_MINIPORT_MAJOR_VERSION == 6 && NDIS_MINIPORT_MINOR_VERSION == 50)
#define NDIS_SUPPORT_NDIS650 1
#endif

#if defined(UM_NDIS640) || defined(NDIS_WRAPPER) || \
    (defined(NDIS_PLATFORM) && (defined(NDIS640) || defined(NDIS640_MINIPORT))) || \
    (NDIS_MINIPORT_MAJOR_VERSION == 6 && NDIS_MINIPORT_MINOR_VERSION == 40)
#define NDIS_SUPPORT_NDIS640 1
#endif

#if defined(UM_NDIS630) || defined(NDIS_WRAPPER) || \
    (defined(NDIS_PLATFORM) && (defined(NDIS630) || defined(NDIS630_MINIPORT))) || \
    (NDIS_MINIPORT_MAJOR_VERSION == 6 && NDIS_MINIPORT_MINOR_VERSION == 30)
#define NDIS_SUPPORT_NDIS630 1
#endif

#if defined(UM_NDIS620) || defined(NDIS_WRAPPER) || \
    (defined(NDIS_PLATFORM) && (defined(NDIS620) || defined(NDIS620_MINIPORT))) || \
    (NDIS_MINIPORT_MAJOR_VERSION == 6 && NDIS_MINIPORT_MINOR_VERSION == 20)
#define NDIS_SUPPORT_NDIS620 1
#endif

#if defined(UM_NDIS61) || defined(NDIS_WRAPPER) || \
    (defined(NDIS_PLATFORM) && (defined(NDIS61) || defined(NDIS61_MINIPORT))) || \
    (NDIS_MINIPORT_MAJOR_VERSION == 6 && NDIS_MINIPORT_MINOR_VERSION == 1)
#define NDIS_SUPPORT_NDIS61 1
#endif

#if defined(UM_NDIS60) || defined(NDIS_WRAPPER) || \
    (defined(NDIS_PLATFORM) && (defined(NDIS60) || defined(NDIS60_MINIPORT))) || \
    (NDIS_MINIPORT_MAJOR_VERSION == 6)
#define NDIS_SUPPORT_NDIS6 1
#endif


#ifdef NDIS_SUPPORT_NDIS689
#define NDIS_SUPPORT_NDIS688 1
#else
#define NDIS_SUPPORT_NDIS689 0
#endif

#ifdef NDIS_SUPPORT_NDIS688
#define NDIS_SUPPORT_NDIS687 1
#else
#define NDIS_SUPPORT_NDIS688 0
#endif

#ifdef NDIS_SUPPORT_NDIS687
#define NDIS_SUPPORT_NDIS686 1
#else
#define NDIS_SUPPORT_NDIS687 0
#endif

#ifdef NDIS_SUPPORT_NDIS686
#define NDIS_SUPPORT_NDIS685 1
#else
#define NDIS_SUPPORT_NDIS686 0
#endif

#ifdef NDIS_SUPPORT_NDIS685
#define NDIS_SUPPORT_NDIS684 1
#else
#define NDIS_SUPPORT_NDIS685 0
#endif

#ifdef NDIS_SUPPORT_NDIS684
#define NDIS_SUPPORT_NDIS683 1
#else
#define NDIS_SUPPORT_NDIS684 0
#endif

#ifdef NDIS_SUPPORT_NDIS683
#define NDIS_SUPPORT_NDIS682 1
#else
#define NDIS_SUPPORT_NDIS683 0
#endif

#ifdef NDIS_SUPPORT_NDIS682
#define NDIS_SUPPORT_NDIS681 1
#else
#define NDIS_SUPPORT_NDIS682 0
#endif

#ifdef NDIS_SUPPORT_NDIS681
#define NDIS_SUPPORT_NDIS680 1
#else
#define NDIS_SUPPORT_NDIS681 0
#endif

#ifdef NDIS_SUPPORT_NDIS680
#define NDIS_SUPPORT_NDIS670 1
#else
#define NDIS_SUPPORT_NDIS680 0
#endif

#ifdef NDIS_SUPPORT_NDIS670
#define NDIS_SUPPORT_NDIS660 1
#else
#define NDIS_SUPPORT_NDIS670 0
#endif

#ifdef NDIS_SUPPORT_NDIS660
#define NDIS_SUPPORT_NDIS651 1
#else
#define NDIS_SUPPORT_NDIS660 0
#endif

#ifdef NDIS_SUPPORT_NDIS651
#define NDIS_SUPPORT_NDIS650 1
#else
#define NDIS_SUPPORT_NDIS651 0
#endif

#ifdef NDIS_SUPPORT_NDIS650
#define NDIS_SUPPORT_NDIS640 1
#else
#define NDIS_SUPPORT_NDIS650 0
#endif

#ifdef NDIS_SUPPORT_NDIS640
#define NDIS_SUPPORT_NDIS630 1
#else
#define NDIS_SUPPORT_NDIS640 0
#endif

#ifdef NDIS_SUPPORT_NDIS630
#define NDIS_SUPPORT_NDIS620 1
#else
#define NDIS_SUPPORT_NDIS630 0
#endif

#ifdef NDIS_SUPPORT_NDIS620
#define NDIS_SUPPORT_NDIS61 1
#else
#define NDIS_SUPPORT_NDIS620 0
#endif

#ifdef NDIS_SUPPORT_NDIS61
#define NDIS_SUPPORT_NDIS6 1
#else
#define NDIS_SUPPORT_NDIS61 0
#endif

#ifndef NDIS_SUPPORT_NDIS6
#define NDIS_SUPPORT_NDIS6 0
#endif
