import pandas as pd
import plotly.express as px
import plotly.graph_objects as go
import dash
import dash_core_components as dcc
import dash_html_components as html
import os

rm_first_path = 'python/data/rm_first/data.csv'
find_all_path = 'python/data/find_all/data.csv'

rm_first_df = pd.read_csv(rm_first_path)
find_all_df = pd.read_csv(find_all_path)

rm_first_df['prob12'] = rm_first_df.apply(lambda row: row.setless12 / (row.set12 + row.setless12), axis=1)
rm_first_df['prob15'] = rm_first_df.apply(lambda row: row.setless15 / (row.set15 + row.setless15), axis=1)
rm_first_df['prob18'] = rm_first_df.apply(lambda row: row.setless18 / (row.set18 + row.setless18), axis=1)

scatter12 = px.scatter(rm_first_df, x="deals", y="prob12")
scatter15 = px.scatter(rm_first_df, x="deals", y="prob15")
scatter18 = px.scatter(rm_first_df, x="deals", y="prob18")

prob_all_18 = [0] * 24
prob_all_15 = [0] * 24
prob_all_12 = [0] * 24

for step in range(24):
    setless18 = find_all_df.loc[find_all_df['deals']==step].loc[find_all_df['hand_size']==18].loc[find_all_df['sets']==0]['count'].sum()
    set18 = find_all_df.loc[find_all_df['deals']==step].loc[find_all_df['hand_size']==18].loc[find_all_df['sets']!=0]['count'].sum()
    prob_all_18[step] =  setless18 / (set18 + setless18)

for step in range(24):
    setless15 = find_all_df.loc[find_all_df['deals']==step].loc[find_all_df['hand_size']==15].loc[find_all_df['sets']==0]['count'].sum() 
    set15 = find_all_df.loc[find_all_df['deals']==step].loc[find_all_df['hand_size']==15].loc[find_all_df['sets']!=0]['count'].sum()
    prob_all_15[step] = setless15 / (set15 + setless15)

for step in range(24):
    setless12 = find_all_df.loc[find_all_df['deals']==step].loc[find_all_df['hand_size']==12].loc[find_all_df['sets']==0]['count'].sum() 
    set12 = find_all_df.loc[find_all_df['deals']==step].loc[find_all_df['hand_size']==12].loc[find_all_df['sets']!=0]['count'].sum()
    prob_all_12[step] = setless12 / (set12 + setless12)

deals = [i for i in range(24)]
data_all = {'deals':deals, 'prob12': prob_all_12, 'prob15':prob_all_15, 'prob18':prob_all_18}
df_all = pd.DataFrame(data_all)
scatter_all_18 = px.scatter(df_all, x="deals", y="prob18")
scatter_all_15 = px.scatter(df_all, x="deals", y="prob15")
scatter_all_12 = px.scatter(df_all, x="deals", y="prob12")
"""
total_sets = go.Figure()

for step in range(23):
    total_sets.add_trace(
        go.Histogram(
            visible = False,
            x = find_all_df.loc[find_all_df['deals']==step].loc[find_all_df['hand_size']==15]["sets"],
            name = "v = " + str(step),
            autobinx = False,
            xbins = dict(
                start = 0,
                end = 8,
                size = 1,
                ),
            histnorm = "probability",
            )
        )


total_sets.data[0].visible = True

steps = []
for i in range(len(total_sets.data)):
    step = dict(
        method = "restyle",
        args = ["visible", [False] * len(total_sets.data)],
        )
    step["args"][1][i] = True
    steps.append(step)

sliders = [dict(
    active = 0,
    currentvalue = {"prefix": "deals: "},
    pad = {"t": 23},
    steps = steps,
    )]

total_sets.update_layout(
    sliders = sliders
)

prob15all = [0]*23
newdeal = [i for i in range(1, 24)]

for x in range(23):
    prob15all[x] = len(find_all_df.loc[find_all_df['hand_size']==15].loc[find_all_df['deals']==(x+1)].loc[find_all_df['sets']==0].index) / len(find_all_df.loc[find_all_df['hand_size']==15].loc[find_all_df['deals']==(x+1)].loc[find_all_df['sets']!=0].index)

data_all = {'deals':newdeal, 'prob15':prob15all}

df_all = pd.DataFrame(data_all)

scatter15_all = px.scatter(df_all, x="deals", y="prob15")
"""
app = dash.Dash(__name__)

app.layout = html.Div(children=[
    html.H1(children = 'Results'),

    dcc.Graph(
        id='12',
        figure=scatter12
        ),

    dcc.Graph(
        id='12_all',
        figure=scatter_all_12
        ),

    dcc.Graph(
        id='15',
        figure=scatter15
        ),

    dcc.Graph(
        id='15_all',
        figure=scatter_all_15
        ),

    dcc.Graph(
        id='18',
        figure=scatter18
        ),

    dcc.Graph(
        id='18_all',
        figure=scatter_all_18
        ),
        
"""
    dcc.Graph(
        id='total_sets',
        figure=total_sets
        )
"""
    ])

app.run_server(debug=True)

