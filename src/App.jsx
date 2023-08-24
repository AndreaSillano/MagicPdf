import {useEffect, useState} from "react";
import { Icon } from '@iconify/react';
import filePdf from '@iconify/icons-codicon/file-pdf';
import baselineMerge from '@iconify/icons-ic/baseline-merge';
import trashOutline from '@iconify/icons-mdi/trash-outline';
import loadingTwotoneLoop from '@iconify/icons-line-md/loading-twotone-loop';



import "./App.css";
import {invoke} from "@tauri-apps/api";

function App() {

  const  [typeErr , setTypeError] = useState(false);
  const [hoveredIndex1, setHoveredIndex1] = useState(null);
  const [hoveredIndex2, setHoveredIndex2] = useState(null);
  const  [pdf1, setPdf1] = useState("");
  const  [pdf2, setPdf2] = useState("");
  const  [pdfPath1, setPdfPath1] = useState("");
  const  [pdfPath2, setPdfPath2] = useState("");
  const [loading, setLoading] = useState(false);



    const  loadPdf =async (index) => {
        let [pdfName, pdfPath] =  (await invoke("load_pdf"));
        console.log(pdfName);
        if (pdfName !== ""){
            if (index === 0){
                setPdf1(pdfName);
                setPdfPath1(pdfPath)
            }else{
                setPdf2(pdfName);
                setPdfPath2(pdfPath)
            }
            setTypeError(false);
        }else{
            setTypeError(true);
        }

  }

    const  delete_pdf = (index)=> {
        if (index === 0){
            setPdf1("");
            setPdfPath1("")
        }else{
            setPdf2("");
            setPdfPath2("")
        }
    }

    const  merge_pdf= async ()=> {
        setLoading(true);
        await invoke("merge",{path1: pdfPath1, path2: pdfPath2});
        reset();
        setLoading(false);
    }
    const reset=()=>{
        setPdf1("");
        setPdfPath1("");
        setPdf2("");
        setPdfPath2("");
    }

    return (
        <>
    <div className="container">
        { loading ?<div className="mb-3"><Icon icon={loadingTwotoneLoop} width={100} color="#0F9D58"/> </div> :<>

      <h1>Welcome to Magic Pdf</h1>
      <div className="row">
          <div className="rounded-border">
              <div className="dot-border">
                  <div className="trash-icon"  >
                      {pdf1 !== "" ? <Icon icon={!hoveredIndex1 ? filePdf : trashOutline } color="#db4437" width={70} onMouseEnter={() => setHoveredIndex1(true)}
                                              onMouseLeave ={() => setHoveredIndex1(null)} onClick={()=>{delete_pdf(0)}}/>:<Icon className="insert-pdf" icon={filePdf} width={70} onClick={()=>{loadPdf(0)}}/>}
                  </div>

              </div>
          </div>

          {pdf2 && pdf1 && <Icon className="mergeIcon" icon={baselineMerge} color="#f4b400" rotate={2} width={80}/>}

          <div className="rounded-border">
              <div className="dot-border">
                  <div className="trash-icon">
                      {pdf2 !== "" ? <Icon icon={!hoveredIndex2 ? filePdf : trashOutline } color="#db4437" width={70} onMouseEnter={() => setHoveredIndex2(true)}
                                              onMouseLeave ={() => setHoveredIndex2(null)} onClick={()=>{delete_pdf(1)}}/>:<Icon className="insert-pdf" icon={filePdf} width={70} onClick={()=>{loadPdf(1)}}/>}
                  </div>
              </div>
          </div>
      </div>
        <div className="row">
                {pdf1 && <p className="file-name">{pdf1}</p>}
                {pdf2 &&  <p className="file-name">{pdf2}</p>}
        </div>

        {pdf1 && pdf2 &&
            <div className="rounded-border-output">
            <div className="dot-border-output">
                <div className="trash-icon">
                    {pdf1 && pdf2 ? <Icon icon={filePdf} color="#0F9D58" width={70} onClick={merge_pdf}/>:<Icon icon={filePdf} width={70}/>}
                </div>
            </div>
        </div>
        }

        {typeErr && <p color="#DB4437">Invalid Format</p>}

        </>}
    </div>
            <footer>
            <p>Developed By Andrea Sillano -  v 0.0.1</p>
            </footer>
    </>


  );
}

export default App;
