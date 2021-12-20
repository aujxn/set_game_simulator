import pandas as pd
import numpy as np
import plotly.graph_objects as go
import dash
import dash_core_components as dcc
import dash_html_components as html

find_all_path = 'data.csv'
find_all_df = pd.read_csv(find_all_path)

hand_sizes = ['12', '15', '18', '21']
hand_types = ['ascending', 'descending']
total_types = ['set_count', 'hand_count', 'setless_hand_count']

totals = {}
for hand_size in hand_sizes:
    totals[hand_size] = {}
    for hand_type in hand_types:
        totals[hand_size][hand_type] = {}
        for total_type in total_types:
            totals[hand_size][hand_type][total_type] = [0 for i in range(24)]

for index, row in find_all_df.iterrows():
    hand_count = row['count']
    set_count = hand_count * row['sets']
    hand_size = str(row['hand_size'])
    deals = row['deals']
    hand_type = row['hand_type']

    totals[hand_size][hand_type]['set_count'][deals] += set_count
    totals[hand_size][hand_type]['hand_count'][deals] += hand_count
    if set_count == 0:
        totals[hand_size][hand_type]['setless_hand_count'][deals] += hand_count

probability_scatters = []
avg_sets_scatters = []
plots = ['ascending', 'descending']

for hand_size in hand_sizes[:3]:
    for plot in plots:
        scatter = go.Figure()
        zipped = zip(totals[hand_size][plot]['setless_hand_count'], totals[hand_size][plot]['hand_count'])
        data = []
        for setless, total in zipped:
            if total > 0:
                data.append(setless / total)
        scatter.add_trace(
            go.Scatter(
                x=list(range(24))[-len(data):],
                y=data,
                mode='markers',
                name=plot
                )
            )

        scatter.update_layout(
            title=go.layout.Title(
                text='Probability of a ' + plot + ' ' + hand_size + ' card hand containing no sets',
                xref='paper',
                ),
            xaxis=go.layout.XAxis(
                title=go.layout.xaxis.Title(
                    text='Times cards have been removed from the deck'
                    )
                ),
            yaxis=go.layout.YAxis(
                title=go.layout.yaxis.Title(
                    text='Probability of no sets'
                    )
                ),
            )
        probability_scatters.append(scatter)

for hand_size in hand_sizes:
    for plot in plots:
        scatter = go.Figure()
        if plot == 'descending' and hand_size == '21':
            continue
        zipped = zip(totals[hand_size][plot]['set_count'], totals[hand_size][plot]['hand_count'])
        data = []
        for setless, total in zipped:
            if total > 0:
                data.append(setless / total)
        scatter.add_trace(
            go.Scatter(
                x=list(range(24))[-len(data):],
                y=data,
                mode='markers',
                name=plot
                )
            )

        scatter.update_layout(
            title=go.layout.Title(
                text='Average number of sets found in a ' + plot + ' ' + hand_size + ' card hand',
                xref='paper',
                ),
            xaxis=go.layout.XAxis(
                title=go.layout.xaxis.Title(
                    text='Times cards have been removed from the deck'
                    )
                ),
            yaxis=go.layout.YAxis(
                title=go.layout.yaxis.Title(
                    text='Set count'
                    )
                ),
            )
        avg_sets_scatters.append(scatter)

app = dash.Dash(__name__)

content = [ html.H1(children = 'Results') ]

for plot_id, scatter in enumerate(probability_scatters):
    content.append(
            dcc.Graph(
                id=f'prob{plot_id}',
                figure=scatter,
                )
            )

for plot_id, scatter in enumerate(avg_sets_scatters):
    content.append(
            dcc.Graph(
                id=f'avg{plot_id}',
                figure=scatter,
                )
            )

app.layout = html.Div(children=content)

app.run_server(debug=True)
