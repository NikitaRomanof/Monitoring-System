use structs::AllData;

use iced::widget::pane_grid::{self, PaneGrid};
use iced::widget::{button, column, container, rich_text, scrollable, span, text};
use iced::{color, font, Center, Element, Fill, Font};

use super::structs;


pub struct ViewContainer {
    panes: pane_grid::State<Pane>,
    count: i32
}

#[derive(Debug, Clone)]
pub enum Message {
    Data(pane_grid::Pane, TypeData),
    Resized(pane_grid::ResizeEvent),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeData {
    Cpu,
    Gpu,
    Dram,
    Ram,
    Os,
    Network,
    Empty,
}

impl ViewContainer {
    fn new(prev_count: i32, _data_pype: TypeData) -> Self {
        let state_pane = pane_grid::State::new(Pane::new(_data_pype));
        let rez = ViewContainer {
            panes: state_pane.0,
            count: prev_count + 1
        };
        return rez;
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Data(pane, data_type) => {
                if self.count >= 2 {
                    self.panes.close(*self.panes.iter().last().unwrap().0);
                    self.panes.split(pane_grid::Axis::Vertical, pane, Pane::new(data_type));
                } else {
                    self.panes.split(pane_grid::Axis::Vertical, pane, Pane::new( data_type));
                    self.count += 1;
                }
            }
            Message::Resized(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.resize(split, ratio);
            }
        }
    }

    pub fn view(&self) -> Element<Message> { 
        let pane_grid = PaneGrid::new(&self.panes, |id: pane_grid::Pane, _pane: &Pane, _false_var: bool| {
            pane_grid::Content::new({
                view_content(id, _pane)
            }
        )
            .style(style::pane_style)
        })
        .width(Fill)
        .height(Fill)
        .spacing(10)
        .on_resize(10, Message::Resized);
        container(pane_grid).padding(10).into()
    }
}

impl Default for ViewContainer {
    fn default() -> Self {
        ViewContainer::new(0, TypeData::Empty)
    }
}

#[derive(Debug, Clone)]
struct Pane {
    pub data: AllData,
    pub type_data: TypeData
}

impl Pane {
    fn new(_type_data: TypeData) -> Self {
        Self {
            data: AllData::new(),
            type_data: _type_data
        }
    }
}

fn view_content<'a>(pane: pane_grid::Pane, cur: &Pane) -> Element<'a, Message> {
    let controls : iced::widget::Column<'_, Message> = make_column_content(pane, cur);
    let content =column![controls].spacing(10);
    scrollable(content).into()
}

fn make_column_content<'a>(pane: pane_grid::Pane, cur: &Pane) -> iced::widget::Column<'a, Message> {

    let button = |label, message| {
        button(text(label).width(Fill).align_x(Center).size(16))
            .width(Fill)
            .padding(10)
            .on_press(message)
    };

    match cur.type_data {
        TypeData::Cpu => 
        return column![rich_text([span("CPU").color(color!(0xff0000)).font(Font { weight: font::Weight::Bold, ..Font::default() }),]),
        scrollable(column![
            text(("count logical cores: ").to_owned() + &(&cur.data.cpu_data.count_logical_cores).to_string()),
            text(("count physical cores: ").to_owned() + &(&cur.data.cpu_data.count_physical_cores).to_string()),
            text(("cpu brand: ").to_owned() + &(&cur.data.cpu_data.cpu_brand)),
            text(("cpu architecture: ").to_owned() + &(&cur.data.cpu_data.cpu_arch)),
            text(("count cpu_usage: ").to_owned() + &(&cur.data.cpu_data.global_cpu_usage).to_string()),
            text(("cpu frequency: ").to_owned() + &(&cur.data.cpu_data.speed).to_string() + "Mhz"),
            text(("cpu temperature: ").to_owned() + &(&cur.data.cpu_data.cpu_temp).to_string() + "°C"),]
        )].spacing(30)
          .padding(10)
          .max_width(900),
    TypeData::Gpu => 
        return column![rich_text([span("GPU").color(color!(0xff0000)).font(Font { weight: font::Weight::Bold, ..Font::default() }),]),
        scrollable(column![
            text(("gpu informations: ").to_owned() + &(&cur.data.gpu_data.gpu_data_vulcan).iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),]
        )].spacing(30)
          .padding(10)
          .max_width(900),

    TypeData::Dram => 
          return column![rich_text([span("DRAM").color(color!(0xff0000)).font(Font { weight: font::Weight::Bold, ..Font::default() }),]),
          scrollable(column![
            text((&cur.data.dram_data.disks).iter().map(|x| "name - ".to_owned() + x.0 + "\n" + &(x.1.to_string())).collect::<Vec<_>>().join("\n*************************\n")),
            
            ]
          )].spacing(30)
            .padding(10)
            .max_width(900),

    TypeData::Ram => 
          return column![rich_text([span("RAM").color(color!(0xff0000)).font(Font { weight: font::Weight::Bold, ..Font::default() }),]),
          scrollable(column![
            text(("total memory: ").to_owned() + &(&cur.data.ram_data.total_memory / 1024000).to_string() + "Mb"),
            text(("used memory: ").to_owned() + &(&cur.data.ram_data.used_memory / 1024000).to_string() + "Mb"),
            text(("total swap: ").to_owned() + &(&cur.data.ram_data.total_swap / 1024000).to_string() + "Mb"),
            text(("free swap: ").to_owned() + &(&cur.data.ram_data.free_swap / 1024000).to_string() + "Mb"),
            text(("used swap: ").to_owned() + &(&cur.data.ram_data.used_swap / 1024000).to_string() + "Mb"),
            text(("available memory: ").to_owned() + &(&cur.data.ram_data.available_memory / 1024000).to_string() + "Mb"),]
          )].spacing(30)
            .padding(10)
            .max_width(900),

    TypeData::Os => 
            return column![rich_text([span("OS").color(color!(0xff0000)).font(Font { weight: font::Weight::Bold, ..Font::default() }),]),
            scrollable(column![
            text(("os type: ").to_owned() + &(&cur.data.os_data.os_type).to_string()),
            text(("name os: ").to_owned() + &(&cur.data.os_data.name_os.clone().unwrap_or("unknown".to_owned()))),
            text(("kernel version: ").to_owned() + &(&cur.data.os_data.kernel_version.clone().unwrap_or("unknown".to_owned()))),
            text(("os version: ").to_owned() + &(&cur.data.os_data.os_version.clone().unwrap_or("unknown".to_owned()))),
            text(("distribution: ").to_owned() + &(&cur.data.os_data.distribution)),
            text(("host name: ").to_owned() + &(&cur.data.os_data.host_name.clone().unwrap_or("unknown".to_owned()))),
            text(("cpu arch: ").to_owned() + &(&cur.data.os_data.cpu_arch)),]
            )].spacing(30)
              .padding(10)
              .max_width(900),

    TypeData::Network => 
              return column![rich_text([span("Network").color(color!(0xff0000)).font(Font { weight: font::Weight::Bold, ..Font::default() }),]),
              scrollable(column![
                text(("network informations: ").to_owned() + &(&cur.data.network_data.data_network).iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),]
              )].spacing(30)
                .padding(10)
                .max_width(900),                  
    _ =>
        return column![
        button(
            "CPU",
            Message::Data(pane, TypeData::Cpu)
        ).style(button::primary),
        button(
            "GPU",
            Message::Data(pane, TypeData::Gpu)
        ).style(button::primary),
        button(
            "DRAM",
            Message::Data(pane, TypeData::Dram)
        ).style(button::primary),
        button(
            "RAM",
            Message::Data(pane, TypeData::Ram)
        ).style(button::primary),
        button(
            "OS",
            Message::Data(pane, TypeData::Os)
        ).style(button::primary),
        button(
            "NETWORK",
            Message::Data(pane, TypeData::Network)
        ).style(button::primary)
        ].spacing(15)
         .max_width(300)
         .padding(10),
    }


}

mod style {
    use iced::widget::container;
    use iced::{Border, Theme};

    pub fn pane_style(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();
        container::Style {
            background: Some(palette.background.weak.color.into()),
            border: Border {
                width: 2.0,
                color: palette.primary.strong.color,
                ..Border::default()
            },
            ..Default::default()
        }
    }
}