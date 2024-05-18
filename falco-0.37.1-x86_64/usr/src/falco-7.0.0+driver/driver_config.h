// SPDX-License-Identifier: GPL-2.0-only OR MIT
/*

Copyright (C) 2023 The Falco Authors.

This file is dual licensed under either the MIT or GPL 2. See MIT.txt
or GPL2.txt for full copies of the license.

*/
#pragma once

/* taken from driver/API_VERSION */
#define PPM_API_CURRENT_VERSION_MAJOR 8
#define PPM_API_CURRENT_VERSION_MINOR 0
#define PPM_API_CURRENT_VERSION_PATCH 1

/* taken from driver/SCHEMA_VERSION */
#define PPM_SCHEMA_CURRENT_VERSION_MAJOR 2
#define PPM_SCHEMA_CURRENT_VERSION_MINOR 14
#define PPM_SCHEMA_CURRENT_VERSION_PATCH 0

#include "ppm_api_version.h"

#define DRIVER_VERSION "7.0.0+driver"

#define DRIVER_NAME "falco"

#define DRIVER_DEVICE_NAME "falco"

#define DRIVER_COMMIT ""

#ifndef KBUILD_MODNAME
#define KBUILD_MODNAME DRIVER_NAME
#endif
