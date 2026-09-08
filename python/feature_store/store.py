# #23 Feature store — timestamp-safe retrieval
class FeatureStore:
    def __init__(self): self.data = {}
    def as_of(self, ts): return self.data.get(ts, {})
    def put(self, ts, features): self.data[ts] = features
