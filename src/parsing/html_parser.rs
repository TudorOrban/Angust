use kuchiki::parse_html;
use kuchiki::NodeRef;
use kuchiki::traits::TendrilSink;

pub fn parse_html_content(html: &str) -> NodeRef {
    parse_html().one(html)
}

pub fn traverse(dom: &NodeRef) {
    for node in dom.inclusive_descendants() {
        if let kuchiki::NodeData::Element(data) = &node.data() {
            println!("Element: {:?}", data.name.local);
        } else if let kuchiki::NodeData::Text(text) = &node.data() {
            println!("Text: {:?}", text.borrow());
        }
    }
}