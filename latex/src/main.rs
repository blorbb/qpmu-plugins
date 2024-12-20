use qpmu_plugin::{clone_async, rank, Action, Input, List, ListItem, Plugin, Result};

// mapping from
// https://github.com/joom/latex-unicoder.vim/blob/master/autoload/unicoder.vim
// with ',' removed for simpler parsing.

struct Latex {
    info: Vec<ListItem>,
}

impl Plugin for Latex {
    async fn new(_: String) -> Result<Self> {
        let info: Vec<_> = include_str!("../mapping.csv")
            .lines()
            .map(|line| {
                line.split_once(',')
                    .unwrap_or_else(|| panic!("failed to split line {line}"))
            })
            .map(|(latex, unicode)| {
                ListItem::new(latex)
                    .with_icon_text(unicode)
                    .on_activate(clone_async!(unicode, || {
                        Ok(vec![Action::Close, Action::Copy(unicode)])
                    }))
                    .on_complete(clone_async!(|| Ok(Some(Input::new(unicode)))))
            })
            .collect();

        Ok(Latex { info })
    }

    async fn query(&self, query: String) -> Result<List> {
        let ranking: Vec<_> = rank::rank(&query, &self.info, rank::Weights::with_history())
            .await
            .into_iter()
            .take(100)
            .collect();

        Ok(List::new(ranking).as_grid())
    }
}

fn main() {
    qpmu_plugin::main::<Latex>();
}
