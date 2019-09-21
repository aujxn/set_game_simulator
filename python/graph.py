import pandas as pd
import plotly.express as px
import dash
import dash_core_components as dcc
import dash_html_components as html

def parseline(line, data):
    for a in line.split('"'):
        try:
            data.append(int(a))
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

data = []
deals = [i for i in range(24)]

set12 = []
set15 = []
set18 = []

setless12 = []
setless15 = []
setless18 = []

f=open("python/data.txt", "r")

if f.mode == 'r':
    lines = f.readlines()
    for x in lines:
        data.append(x)

    parseline(data[0], setless12)
    parseline(data[1], set12)
    parseline(data[2], setless15)
    parseline(data[3], set15)
    parseline(data[4], setless18)
    parseline(data[5], set18)

    prob12 = calcProb(setless12, set12)
    prob15 = calcProb(setless15, set15)
    prob18 = calcProb(setless18, set18)

    data = {'deals':deals, 'prob12':prob12, 'prob15':prob15, 'prob18':prob18}
    df = pd.DataFrame(data)

    print(df)

    scatter12 = px.scatter(df, x="deals", y="prob12")
    scatter15 = px.scatter(df, x="deals", y="prob15")
    scatter18 = px.scatter(df, x="deals", y="prob18")

    app = dash.Dash(__name__)
    
    app.layout = html.Div(children=[
        html.H1(children = 'Results'),

        dcc.Graph(
            id='12',
            figure=scatter12
        ),

        dcc.Graph(
            id='15',
            figure=scatter15
        ),

        dcc.Graph(
            id='18',
            figure=scatter18
        )
    ])

    app.run_server(debug=True)