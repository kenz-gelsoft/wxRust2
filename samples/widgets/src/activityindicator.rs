// SPDX-License-Identifier: LGPL-2.0-or-later WITH WxWindows-exception-3.1
//
// wxWidgets Sample (partially/incompletely) ported in Rust.
// Ported by:   KENZ<KENZ.gelsoft@gmail.com>
// Original C++ Version's Copyright is:
/////////////////////////////////////////////////////////////////////////////
// Program:     wxWidgets Widgets Sample
// Name:        activityindicator.cpp
// Purpose:     Part of the widgets sample showing wxActivityIndicator
// Author:      Vadim Zeitlin
// Created:     2015-03-06
// Copyright:   (c) 2015 wxWindows team
/////////////////////////////////////////////////////////////////////////////

use std::os::raw::c_int;
use wx::methods::*;

enum ActivityIndicator {
    Start = wx::ID_HIGHEST as isize,
    Stop,
    IsRunning,
}
impl From<ActivityIndicator> for c_int {
    fn from(w: ActivityIndicator) -> Self {
        w as c_int
    }
}

#[derive(Clone)]
pub struct ActivityIndicatorWidgetsPage {
    pub base: wx::Panel,
    m_sizer_indicator: wx::StaticBoxSizer,
    m_indicator: Option<wx::ActivityIndicator>,
}
impl ActivityIndicatorWidgetsPage {
    pub fn new<P: WindowMethods>(book: &P) -> Self {
        let panel = wx::Panel::builder(Some(book))
            .style(wx::CLIP_CHILDREN | wx::TAB_TRAVERSAL)
            .build();
        let sizer_indicator =
            wx::StaticBoxSizer::new_with_int(wx::HORIZONTAL, Some(&panel), "Activity Indicator");
        ActivityIndicatorWidgetsPage {
            base: panel,
            m_sizer_indicator: sizer_indicator,
            m_indicator: None,
        }
    }
    pub fn create_content(&mut self) {
        let sizer_oper =
            wx::StaticBoxSizer::new_with_int(wx::VERTICAL, Some(&self.base), "&Operations");

        let start_btn = wx::Button::builder(Some(&self.base))
            .id(ActivityIndicator::Start.into())
            .label("&Start")
            .build();
        sizer_oper.add_window_sizerflags(
            Some(&start_btn),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );
        let stop_btn = wx::Button::builder(Some(&self.base))
            .id(ActivityIndicator::Stop.into())
            .label("&Stop")
            .build();
        sizer_oper.add_window_sizerflags(
            Some(&stop_btn),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );

        sizer_oper.add_window_sizerflags(
            Some(
                &wx::StaticText::builder(Some(&self.base))
                    .id(ActivityIndicator::IsRunning.into())
                    .label("Indicator is initializing...")
                    .build(),
            ),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );

        self.recreate_widget();

        let sizer_top = wx::BoxSizer::new(wx::HORIZONTAL);
        sizer_top.add_sizer_sizerflags(
            Some(&sizer_oper),
            wx::SizerFlags::new(0).expand().double_border(wx::ALL),
        );
        sizer_top.add_sizer_sizerflags(
            Some(&self.m_sizer_indicator),
            wx::SizerFlags::new(1).expand().double_border(wx::ALL),
        );

        self.base.set_sizer(Some(&sizer_top), true);
    }
    fn recreate_widget(&mut self) {
        self.m_sizer_indicator.clear(true /* delete windows */);

        self.m_indicator = Some(
            wx::ActivityIndicator::builder(Some(&self.base))
                .id(wx::ID_ANY)
                .style(wx::BORDER_DEFAULT)
                .build(),
        );

        self.m_sizer_indicator.add_stretch_spacer(1);
        self.m_sizer_indicator
            .add_window_sizerflags(self.m_indicator.as_ref(), wx::SizerFlags::new(0).centre());
        self.m_sizer_indicator.add_stretch_spacer(1);
        self.m_sizer_indicator.layout();
    }
}
