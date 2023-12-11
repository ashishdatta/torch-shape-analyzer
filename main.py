class Net(nn.Module):
    def __init__(self, out_classes):
        super(Net, self).__init__()
        self.layers = nn.Sequential(
            nn.Linear(28 * 28, 120),
            nn.ReLU(),
            nn.Linear(120, out_classes)
        )
    def forward(self, x):
        x = x.reshape(x.shape[0], -1)
        x = self.layers(x)
        return x
model = Net(out_classes=10)
data = dataset.MNIST('./data', train=True, transform=[ToTensor()])
loader = DataLoader(data, batch_size=16)
for epoch in range(10):
    for batch, label in loader:
        output = model(batch)
        loss = F.nll_loss(output, label)
        loss.backward()