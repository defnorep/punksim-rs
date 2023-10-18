import cytoscape from "https://cdnjs.cloudflare.com/ajax/libs/cytoscape/3.26.0/cytoscape.esm.min.js";

const elements = JSON.parse(document.getElementById("graph").innerHTML);
const cy = cytoscape({
  container: document.getElementById("render"),
  elements,
  style: [
    {
      selector: "node",
      style: {
        color: "#fff",
        "font-size": "16px",
        "background-color": "#18e018",
        label: "data(label)",
      },
    },

    {
      selector: "edge",
      style: {
        "curve-style": "haystack",
        "haystack-radius": 0,
        opacity: 0.35,
        "line-color": "#a2efa2",
      },
    },
  ],
  layout: {
    name: "grid",
    rows: 2,
  },
});
