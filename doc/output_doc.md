# Collected workflows

We have added a collection of workflows stored in the `workflows` directory. These workflows are designed to be run from the workflow directory and are intended to be used as a starting point for your own analyses. They are fully commented and can be used as a "follow-along" guide to the gretl toolkit. The workflows answer individual questions and are designed to be run in sequence. 

Comment: Workflows are no bash scripts. They are markdown files that contain a series of commands that can be run in sequence. A list of the workflows is provided below:

- How does my graph saturate?  saturation.md
- Show me the local complexity of the graph: nwindow.md
- How much is core, shell and private sequence do I have?
- On a genome scale, which regions are similar to each other and which are not? window.md
- I have several graphs, show me the differences between them? (histogram for each feature) 
- I have several graphs, show me the differences between them? (heatmap - representation)
- Of the graphs I provided, which features span the most interesting range of values? (Scatter plot)
- Show me scatter plot of these two features (name feature here)? (Scatter plot)
- ein graph, pfad stats und dann histogramm aller pfade 
- Correlieren irgendwelche Features in meinem Datenset? (Clustermap - representation)
- 
1 prerun for stats, later do the rest 


python1.py -l list.graph.txt -x "Unique nodes" -y "Unique edges" -o plot1