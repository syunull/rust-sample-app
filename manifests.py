#!/usr/bin/env python
from constructs import Construct
from cdk8s import App, Chart
from cdk8s_plus_24 import Deployment


class MyChart(Chart):
    def __init__(self, scope: Construct, id: str):
        super().__init__(scope, id)

        self.deployment = Deployment(
            self, "deployment", containers=[{"image": "nginx"}]
        )


app = App()
MyChart(app, "hello")

app.synth()
