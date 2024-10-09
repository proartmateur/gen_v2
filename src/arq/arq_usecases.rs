use super::arq_item::ArqItem;
use super::arq::Arq;


/// Function to find an ArqItem by its `option` or `short_option`.
/// The lifetime `'a` ensures that the reference to the `ArqItem` 
/// is valid for as long as the `Arq` vector lives.
pub fn find_arq_item_by_option<'a>(arq: &'a Arq, search_option: &str) -> Option<&'a ArqItem> {
    arq.iter().find(|item| item.option == search_option || item.short_option == search_option)
}
