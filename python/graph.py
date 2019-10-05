import pandas as pd
import plotly.express as px
import plotly.graph_objects as go
import dash
import dash_core_components as dcc
import dash_html_components as html
import os

def parseline(line, data):
    i = 0;
    for a in line.split(' '):
        try:
            data[i] += int(a)
            i += 1
        except ValueError:
            pass

def calcProb(setless, with_sets):
    prob = []
    for i in range(len(setless)):
        if with_sets[i] == 0:
            prob.append(0)
        else:
            prob.append(float(setless[i]) / float(with_sets[i]))
    return prob

deals = [i for i in range(24)]

set12 = [0]*24
set15 = [0]*24
set18 = [0]*24

setless12 = [0]*24
setless15 = [0]*24
setless18 = [0]*24

path = 'python/data/rm_first/'

for filename in os.listdir(path):

    f=open(path + filename)

    data = []
    lines = f.readlines()
    for x in lines:
        data.append(x)

    parseline(data[0], setless12)
    parseline(data[1], setless15)
    parseline(data[2], setless18)
    parseline(data[3], set12)
    parseline(data[4], set15)
    parseline(data[5], set18)

f = open(path+"data.csv", "w+")
f.write("deals,setless12,set12,setless15,set15,setless18,set18\n")
for i in range(24):
    f.write(str(i) + ',' + str(setless12[i]) + ',' + str(set12[i]) + ',' + str(setless15[i]) + ',' + str(set15[i]) + ',' + str(setless18[i]) + ',' + str(set18[i]) + '\n')
"""
prob12 = calcProb(setless12, set12)
prob15 = calcProb(setless15, set15)
prob18 = calcProb(setless18, set18)

data = {'deals':deals, 'prob12':prob12, 'prob15':prob15, 'prob18':prob18}
df = pd.DataFrame(data)

scatter12 = px.scatter(df, x="deals", y="prob12")
scatter15 = px.scatter(df, x="deals", y="prob15")
scatter18 = px.scatter(df, x="deals", y="prob18")

find_all_df = pd.read_csv("python/data/find_all/data.csv")


prob_all_18 = [0] * 24
prob_all_15 = [0] * 24
prob_all_12 = [0] * 24

for step in range(24):
    prob_all_18[step] = find_all_df.loc[find_all_df['deals']==step].loc[find_all_df['hand_size']==18].loc[find_all_df['sets']==0]['count'].sum() / find_all_df.loc[find_all_df['deals']==step].loc[find_all_df['hand_size']==18].loc[find_all_df['sets']!=0]['count'].sum()

for step in range(24):
    prob_all_15[step] = find_all_df.loc[find_all_df['deals']==step].loc[find_all_df['hand_size']==15].loc[find_all_df['sets']==0]['count'].sum() / find_all_df.loc[find_all_df['deals']==step].loc[find_all_df['hand_size']==15].loc[find_all_df['sets']!=0]['count'].sum()

for step in range(24):
    prob_all_12[step] = find_all_df.loc[find_all_df['deals']==step].loc[find_all_df['hand_size']==12].loc[find_all_df['sets']==0]['count'].sum() / find_all_df.loc[find_all_df['deals']==step].loc[find_all_df['hand_size']==12].loc[find_all_df['sets']!=0]['count'].sum()

data_all = {'deals':deals, 'prob12': prob_all_12, 'prob15':prob_all_15, 'prob18':prob_all_18}
df_all = pd.DataFrame(data_all)
scatter_all_18 = px.scatter(df_all, x="deals", y="prob18")
scatter_all_15 = px.scatter(df_all, x="deals", y="prob15")
scatter_all_12 = px.scatter(df_all, x="deals", y="prob12")
"""
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
        
    dcc.Graph(
        id='total_sets',
        figure=total_sets
        )
    ])

app.run_server(debug=True)
"""

