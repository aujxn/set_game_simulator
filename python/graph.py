import pandas as pd
import numpy as np
import plotly.graph_objects as go
import dash
import dash_core_components as dcc
import dash_html_components as html

rm_first_path = 'python/data/rm_first/data.csv'
find_all_path = 'python/data/find_all/data.csv'

rm_first_df = pd.read_csv(rm_first_path)
find_all_df = pd.read_csv(find_all_path)

rm_rand_df = pd.DataFrame(index=[i for i in range(24)])

hand_sizes = ['12', '15', '18', '21']
set_types = ['cube', 'face', 'edge', 'vertex']
setless_hand_count_scatters = []
probability_scatters = []
avg_sets_scatters = []
set_class_scatters = []

for i in range(3):
    probs = []

    for index, row in rm_first_df.iterrows():
        setless = row['setless' + hand_sizes[i]]
        total = row['set' + hand_sizes[i]] + setless
        prob = 0
        if total != 0:
            prob = setless / total
        probs.append(prob)

    rm_first_df['prob_no_sets' + hand_sizes[i]] = probs

total_hands = [[0 for i in range(24)] for i in range(4)]
total_set_count = [[0 for i in range(24)] for i in range(4)]
total_cubes = [[0 for i in range(24)] for i in range(4)]
total_faces = [[0 for i in range(24)] for i in range(4)]
total_edges = [[0 for i in range(24)] for i in range(4)]
total_vertices = [[0 for i in range(24)] for i in range(4)]
hands_without_sets = [[0 for i in range(24)] for i in range(3)]

for index, row in find_all_df.iterrows():
    hand_count = row['count']
    cubes = hand_count * row['cubes']
    faces = hand_count * row['faces']
    edges = hand_count * row['edges']
    vertices = hand_count * row['vertices']
    set_count = cubes + faces + edges + vertices
    hand_size = np.int_((row['hand_size'] - 12) / 3)
    deals = np.int_(row['deals'])

    total_set_count[hand_size][deals] += set_count
    total_hands[hand_size][deals] += hand_count
    total_cubes[hand_size][deals] += cubes
    total_faces[hand_size][deals] += faces
    total_edges[hand_size][deals] += edges
    total_vertices[hand_size][deals] += vertices

    if set_count == 0:
        hands_without_sets[hand_size][deals] += hand_count

for i in range(4):
    rm_rand_df['total_hands' + hand_sizes[i]] = total_hands[i]
    rm_rand_df['total_set_count' + hand_sizes[i]] = total_set_count[i]
    rm_rand_df['cubes' + hand_sizes[i]] = total_cubes[i]
    rm_rand_df['faces' + hand_sizes[i]] = total_faces[i]
    rm_rand_df['edges' + hand_sizes[i]] = total_edges[i]
    rm_rand_df['vertices' + hand_sizes[i]] = total_vertices[i]
    if i != 3:
        rm_rand_df['hands_without_sets' + hand_sizes[i]] = hands_without_sets[i]

avg_set_count = [[0 for i in range(24)] for i in range(4)]
prob_no_sets = [[0 for i in range(24)] for i in range(3)]
cube_prop = [[0 for i in range(24)] for i in range(4)]
face_prop = [[0 for i in range(24)] for i in range(4)]
edge_prop = [[0 for i in range(24)] for i in range(4)]
vertex_prop = [[0 for i in range(24)] for i in range(4)]


for index, row in rm_rand_df.iterrows():
    for i in range(4):
        if row['total_hands' + hand_sizes[i]] != 0:
            avg_set_count[i][index] = row['total_set_count' + hand_sizes[i]] / row['total_hands' + hand_sizes[i]]
            cube_prop[i][index] = row['cubes' + hand_sizes[i]] / row['total_set_count' + hand_sizes[i]]
            face_prop[i][index] = row['faces' + hand_sizes[i]] / row['total_set_count' + hand_sizes[i]]
            edge_prop[i][index] = row['edges' + hand_sizes[i]] / row['total_set_count' + hand_sizes[i]]
            vertex_prop[i][index] = row['vertices' + hand_sizes[i]] / row['total_set_count' + hand_sizes[i]]

            if i != 3:
                prob_no_sets[i][index] = row['hands_without_sets' + hand_sizes[i]] / row['total_hands' + hand_sizes[i]]
            
for i in range(4):
    rm_rand_df['avg_set_count' + hand_sizes[i]] = avg_set_count[i]
    rm_rand_df['cube_prop' + hand_sizes[i]] = cube_prop[i]
    rm_rand_df['face_prop' + hand_sizes[i]] = face_prop[i]
    rm_rand_df['edge_prop' + hand_sizes[i]] = edge_prop[i]
    rm_rand_df['vertex_prop' + hand_sizes[i]] = vertex_prop[i]

    if i != 3:
        rm_rand_df['prob_no_sets' + hand_sizes[i]] = prob_no_sets[i]
                
for i in range(3):
    setless_hand_count_scatters.append(go.Figure())

    filtered_rand = rm_rand_df.loc[rm_rand_df['hands_without_sets' + hand_sizes[i]] != 0]

    setless_hand_count_scatters[i].add_trace(
            go.Scatter(
                x=filtered_rand.index,
                y=filtered_rand['hands_without_sets' + hand_sizes[i]],
                mode='markers',
                name='with random set removed'
                )
            )

    setless_hand_count_scatters[i].update_layout(
            title=go.layout.Title(
                text='Counts of setless ' + hand_sizes[i] + ' card hand containing no sets',
                xref='paper',
                ),
            xaxis=go.layout.XAxis(
                title=go.layout.xaxis.Title(
                    text='Times cards have been removed from the deck'
                    )
                ),
            yaxis=go.layout.YAxis(
                title=go.layout.yaxis.Title(
                    text='Number of hands'
                    )
                ),
            )

for i in range(3):
    probability_scatters.append(go.Figure())

    filtered_rand = rm_rand_df.loc[rm_rand_df['prob_no_sets' + hand_sizes[i]] != 0]
    filtered_first = rm_first_df.loc[rm_first_df['prob_no_sets' + hand_sizes[i]] != 0]

    probability_scatters[i].add_trace(
            go.Scatter(
                x=filtered_first.index,
                y=filtered_first['prob_no_sets' + hand_sizes[i]],
                mode='markers',
                name='with first set found removed'
                )
            )

    probability_scatters[i].add_trace(
            go.Scatter(
                x=filtered_rand.index,
                y=filtered_rand['prob_no_sets' + hand_sizes[i]],
                mode='markers',
                name='with random set removed'
                )
            )

    probability_scatters[i].update_layout(
            title=go.layout.Title(
                text='Probability of a ' + hand_sizes[i] + ' card hand containing no sets',
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

probability_scatters.append(go.Figure())

filtered_rand = rm_rand_df.drop([0, 1, 23])
filtered_first = rm_first_df.drop([0, 1, 23])

probability_scatters[3].add_trace(
        go.Scatter(
            x=filtered_first.index,
            y=filtered_first['prob_no_sets18'],
            mode='markers',
            name='with first set found removed'
            )
        )

probability_scatters[3].add_trace(
        go.Scatter(
            x=filtered_rand.index,
            y=filtered_rand['prob_no_sets18'],
            mode='markers',
            name='with random set removed'
            )
        )

probability_scatters[3].update_layout(
        title=go.layout.Title(
            text='Probability of a 18 card hand containing no sets (outlier removed)',
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

for i in range(4):
    set_class_scatters.append(go.Figure())
    filtered_rand = rm_rand_df.loc[rm_rand_df['avg_set_count' + hand_sizes[i]] != 0]

    for set_type in set_types:
        set_class_scatters[i].add_trace(
                go.Scatter(
                    x=filtered_rand.index,
                    y=filtered_rand[set_type + '_prop' + hand_sizes[i]],
                    mode='markers',
                    name= set_type,
                    )
                )

    set_class_scatters[i].update_layout(
            title=go.layout.Title(
                text='Proportion of set types in a ' + hand_sizes[i] + ' card hand',
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

for i in range(4):
    avg_sets_scatters.append(go.Figure())

    filtered_rand = rm_rand_df.loc[rm_rand_df['avg_set_count' + hand_sizes[i]] != 0]
    avg_sets_scatters[i].add_trace(
            go.Scatter(
                x=filtered_rand.index,
                y=filtered_rand['avg_set_count' + hand_sizes[i]],
                mode='markers',
                name='avg sets'
                )
            )

    avg_sets_scatters[i].update_layout(
            title=go.layout.Title(
                text='Average number of sets found in a ' + hand_sizes[i] + ' card hand',
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

app = dash.Dash(__name__)

content = [ html.H1(children = 'Results') ]

for i in range(4):
    content.append(
            dcc.Graph(
                id='prob' + hand_sizes[i],
                figure=probability_scatters[i],
                )
            )

for i in range(3):
    content.append(
            dcc.Graph(
                id='setless_hand_count' + hand_sizes[i],
                figure=setless_hand_count_scatters[i],
                )
            )

for i in range(4):
    content.append(
            dcc.Graph(
                id='avg' + hand_sizes[i],
                figure=avg_sets_scatters[i],
                )
            )

for i in range(4):
    content.append(
            dcc.Graph(
                id='types' + hand_sizes[i],
                figure=set_class_scatters[i],
                )
            )
app.layout = html.Div(children=content)

app.run_server(debug=True)
