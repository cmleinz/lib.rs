use crate::lib::arxiv;
use tui::widgets::ListState;

pub enum Event<I> {
    Input(I),
    Tick,
}

pub enum MenuItem {
    Home,
    Search,
    Favorites,
    Settings,
}

#[derive(PartialEq)]
pub enum InputState {
    NormalMode,
    InsertMode,
}

impl InputState {
    pub fn to_string(&self) -> String {
        match self {
            InputState::NormalMode => "= NORMAL MODE = (Select a paper)".into(),
            InputState::InsertMode => "= INSERT MODE = (Search for results)".into(),
        }
    }
}

impl MenuItem {
    pub const TITLES: &'static [&'static str; 4] = &["Home", "Search", "Favorite", "Settings"];
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Search => 1,
            MenuItem::Favorites => 2,
            MenuItem::Settings => 3,
        }
    }
}

pub enum HomePanel {
    SearchBar,
    ListView,
}

pub struct TuiState {
    page: MenuItem,
    pub input: String,
    pub input_state: InputState,
    pub list_state: ListState,
    pub active_home_panel: HomePanel,
    pub data: Option<Vec<arxiv::Entry>>,
    pub client: arxiv::Client,
}

impl Default for TuiState {
    fn default() -> TuiState {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        TuiState {
            page: MenuItem::Home,
            input: String::new(),
            input_state: InputState::InsertMode,
            active_home_panel: HomePanel::SearchBar,
            list_state,
            data: None,
            client: arxiv::Client::default(),
        }
    }
}

impl TuiState {
    pub fn search(&mut self, start: u32, max: u32) {
        let query_input = self.get_query_str();
        let query_str = format!(
            "http://export.arxiv.org/api/query?search_query=all:{}&start={}&max_results={}",
            query_input, start, max
        );
        // This is all being handled unsafely at present, need to add good error-handling
        // This client.client API is not good.
        let resp = self.client.client.get(&query_str).send().unwrap();
        let text = resp.text().unwrap();
        let doc = roxmltree::Document::parse(&text).unwrap();
        let entry = doc
            .descendants()
            .filter(|n| n.has_tag_name("entry"))
            .map(|n| {
                let title = arxiv::get_title(&n);
                let summary = arxiv::get_summary(&n);
                let pdf_link = arxiv::get_pdf_link(&n);
                let authors = arxiv::get_authors(&n);
                arxiv::Entry {
                    title,
                    summary,
                    pdf_link,
                    authors,
                }
            })
            .collect::<Vec<arxiv::Entry>>();
        self.data = Some(entry);
    }

    fn get_query_str(&self) -> String {
        let mut q_string = self.input.clone();
        q_string = q_string.replace("(", "%28");
        q_string = q_string.replace(")", "%29");
        q_string = q_string.replace(" ", "+");
        q_string = q_string.replace("\"", "%22");
        q_string
    }

    pub fn data_len(&self) -> usize {
        match &self.data {
            Some(data) => data.len(),
            None => 0,
        }
    }
    pub fn get_selected_entry(&self) -> Option<&arxiv::Entry> {
        if let Some(u) = self.list_state.selected() {
            if let Some(data) = &self.data {
                Some(&data[u])
            } else {
                None
            }
        } else {
            None
        }
    }
}
