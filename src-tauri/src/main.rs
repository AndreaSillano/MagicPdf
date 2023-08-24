// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::BTreeMap;
use nfd::Response;
use lopdf::dictionary;
use lopdf::content::{Content, Operation};
use lopdf::{Document, Object, ObjectId, Stream, Bookmark};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
fn load_pdf() -> (String,String) {
    let result = nfd::dialog().open().unwrap_or_else(|e| {
        panic!("{}",e);
    });
    match result {
        Response::Okay(file_path)=>{
            let name_split:Vec<_> = file_path.split("/").collect();
            let name_ext: Vec<_> = name_split[name_split.len()-1].split(".").collect();
            let ext = name_ext[name_ext.len()-1];
            if  ext == "PDF" || ext == "pdf"{
                let name = remove_extension(file_path.as_str());


                return (name.to_string(), file_path.to_string() );
            }

            ("".to_string(), "".to_string() )

        },
        Response::Cancel=>{
            ("".to_string(), "".to_string())
        }
        _=>{("".to_string(), "".to_string())}
    }
}
#[tauri::command]
fn merge(path1: String, path2: String){
    if let Response::Okay(path) = nfd::dialog_save().open().unwrap_or_else(|e|{panic!("{}",e);}) {
        merge_pdf(path, path1, path2).expect("Unable To Save");
    }

}
fn remove_extension(file_name: &str) -> String {
    let name_split:Vec<_> = file_name.split("/").collect();
    let name = name_split[name_split.len()-1].to_string();
    let parts: Vec<&str> = name.split('.').collect();
    if parts.len() > 1 {
        parts[0..parts.len() - 1].join(".")
    } else {
        file_name.to_string()
    }
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, load_pdf, merge])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
fn merge_pdf(save_path:String, path1:String, path2: String ) -> std::io::Result<()>  {
    let path_0 = std::path::Path::new(path1.as_str());
    let path_1 = std::path::Path::new(path2.as_str());
    let mut doc_0 = Document::load(path_0).unwrap();
    let mut doc_1 = Document::load(path_1).unwrap();
    let documents = vec![doc_0, doc_1];


    // Define a starting max_id (will be used as start index for object_ids)
    let mut max_id = 1;
    let mut pagenum = 1;
    // Collect all Documents Objects grouped by a map
    let mut documents_pages = BTreeMap::new();
    let mut documents_objects = BTreeMap::new();
    let mut document = Document::with_version("1.5");

    for mut doc in documents {
        let mut first = false;
        doc.renumber_objects_with(max_id);

        max_id = doc.max_id + 1;

        documents_pages.extend(
            doc
                .get_pages()
                .into_iter()
                .map(|(_, object_id)| {
                    if !first {
                        let bookmark = Bookmark::new(String::from(format!("Page_{}", pagenum)), [0.0, 0.0, 1.0], 0, object_id);
                        document.add_bookmark(bookmark, None);
                        first = true;
                        pagenum += 1;
                    }

                    (
                        object_id,
                        doc.get_object(object_id).unwrap().to_owned(),
                    )
                })
                .collect::<BTreeMap<ObjectId, Object>>(),
        );
        documents_objects.extend(doc.objects);
    }

    // Catalog and Pages are mandatory
    let mut catalog_object: Option<(ObjectId, Object)> = None;
    let mut pages_object: Option<(ObjectId, Object)> = None;

    // Process all objects except "Page" type
    for (object_id, object) in documents_objects.iter() {
        // We have to ignore "Page" (as are processed later), "Outlines" and "Outline" objects
        // All other objects should be collected and inserted into the main Document
        match object.type_name().unwrap_or("") {
            "Catalog" => {
                // Collect a first "Catalog" object and use it for the future "Pages"
                catalog_object = Some((
                    if let Some((id, _)) = catalog_object {
                        id
                    } else {
                        *object_id
                    },
                    object.clone(),
                ));
            }
            "Pages" => {
                // Collect and update a first "Pages" object and use it for the future "Catalog"
                // We have also to merge all dictionaries of the old and the new "Pages" object
                if let Ok(dictionary) = object.as_dict() {
                    let mut dictionary = dictionary.clone();
                    if let Some((_, ref object)) = pages_object {
                        if let Ok(old_dictionary) = object.as_dict() {
                            dictionary.extend(old_dictionary);
                        }
                    }

                    pages_object = Some((
                        if let Some((id, _)) = pages_object {
                            id
                        } else {
                            *object_id
                        },
                        Object::Dictionary(dictionary),
                    ));
                }
            }
            "Page" => {}     // Ignored, processed later and separately
            "Outlines" => {} // Ignored, not supported yet
            "Outline" => {}  // Ignored, not supported yet
            _ => {
                document.objects.insert(*object_id, object.clone());
            }
        }
    }

    // If no "Pages" object found abort
    if pages_object.is_none() {
        println!("Pages root not found.");

        return Ok(());
    }

    // Iterate over all "Page" objects and collect into the parent "Pages" created before
    for (object_id, object) in documents_pages.iter() {
        if let Ok(dictionary) = object.as_dict() {
            let mut dictionary = dictionary.clone();
            dictionary.set("Parent", pages_object.as_ref().unwrap().0);

            document
                .objects
                .insert(*object_id, Object::Dictionary(dictionary));
        }
    }

    // If no "Catalog" found abort
    if catalog_object.is_none() {
        println!("Catalog root not found.");

        return Ok(());
    }

    let catalog_object = catalog_object.unwrap();
    let pages_object = pages_object.unwrap();

    // Build a new "Pages" with updated fields
    if let Ok(dictionary) = pages_object.1.as_dict() {
        let mut dictionary = dictionary.clone();

        // Set new pages count
        dictionary.set("Count", documents_pages.len() as u32);

        // Set new "Kids" list (collected from documents pages) for "Pages"
        dictionary.set(
            "Kids",
            documents_pages
                .into_iter()
                .map(|(object_id, _)| Object::Reference(object_id))
                .collect::<Vec<_>>(),
        );

        document
            .objects
            .insert(pages_object.0, Object::Dictionary(dictionary));
    }

    // Build a new "Catalog" with updated fields
    if let Ok(dictionary) = catalog_object.1.as_dict() {
        let mut dictionary = dictionary.clone();
        dictionary.set("Pages", pages_object.0);
        dictionary.remove(b"Outlines"); // Outlines not supported in merged PDFs

        document
            .objects
            .insert(catalog_object.0, Object::Dictionary(dictionary));
    }

    document.trailer.set("Root", catalog_object.0);

    // Update the max internal ID as wasn't updated before due to direct objects insertion
    document.max_id = document.objects.len() as u32;

    // Reorder all new Document objects
    document.renumber_objects();

    //Set any Bookmarks to the First child if they are not set to a page
    document.adjust_zero_pages();

    //Set all bookmarks to the PDF Object tree then set the Outlines to the Bookmark content map.
    if let Some(n) = document.build_outline() {
        if let Ok(x) = document.get_object_mut(catalog_object.0) {
            if let Object::Dictionary(ref mut dict) = x {
                dict.set("Outlines", Object::Reference(n));
            }
        }
    }

    document.compress();

    // Save the merged PDF
    // Store file in current working directory.
    // Note: Line is excluded when running tests

    document.save(save_path+".pdf").unwrap();

    Ok(())
}