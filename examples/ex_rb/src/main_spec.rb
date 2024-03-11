require "./main"

describe "#hello_name" do
    it "returns the Hello World Ariful" do
      expect(hello_name("Ariful")).to eql("Hello World and hello Ariful")
    end
end